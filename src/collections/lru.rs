use alloc::boxed::Box;
use alloc::collections::VecDeque;

/// Implements an least recently used cache. It has a fixed size and will remove
/// the least recently used item when the size is reached. The least recently used
/// item is always the first item in the queue. The item is removed by calling the
/// function provided to [`LruCache::with_evict`]. The default action is to drop the
/// item.
pub struct LruCache<V> {
    max_size: usize,
    data: VecDeque<V>,
    on_evict: Box<dyn Fn(V)>,
}

impl<V> LruCache<V> {
    pub fn new(size: usize) -> Self {
        Self::with_evict(size, |v| drop(v))
    }

    pub fn with_evict(size: usize, on_evict: impl Fn(V) + 'static) -> Self {
        Self {
            max_size: size,
            data: VecDeque::with_capacity(size),
            on_evict: Box::new(on_evict),
        }
    }

    pub fn find<P>(&mut self, predicate: P) -> Option<&V>
    where
        P: FnMut(&V) -> bool,
    {
        if let Some(position) = self.data.iter().position(predicate) {
            let item = self.data.remove(position).unwrap();
            self.data.push_front(item);
            return Some(&self.data[0]);
        }
        None
    }

    pub fn insert(&mut self, item: V) {
        if self.data.len() >= self.max_size {
            if let Some(item) = self.data.pop_back() {
                self.evict(item);
            }
        }
        self.data.push_front(item);
    }

    #[inline]
    fn evict(&self, item: V) {
        (self.on_evict)(item)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<V> Drop for LruCache<V> {
    fn drop(&mut self) {
        while let Some(e) = self.data.pop_back() {
            self.evict(e);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::VecDeque;
    use crate::sync::atomic::{AtomicUsize, Ordering};
    use crate::sync::Arc;

    use super::LruCache;

    #[test]
    fn test_lru_new_is_empty() {
        let lru = LruCache::<u8>::new(10);
        assert_eq!(VecDeque::from([]), lru.data);
    }

    #[test]
    fn test_lru_insert_reverse() {
        let mut lru = LruCache::<u8>::new(10);
        lru.insert(0);
        lru.insert(1);
        lru.insert(2);
        lru.insert(2);
        lru.insert(3);
        assert_eq!(VecDeque::from([3, 2, 2, 1, 0]), lru.data);
    }

    #[test]
    fn test_lru_insert() {
        let mut lru = LruCache::<u8>::new(10);
        for i in 0_u8..10 {
            lru.insert(i);
        }
        assert_eq!(VecDeque::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), lru.data);
        let &item = lru.find(|&v| v == 4).unwrap();
        assert_eq!(4, item);
        assert_eq!(VecDeque::from([4, 9, 8, 7, 6, 5, 3, 2, 1, 0]), lru.data);
    }

    #[test]
    fn test_lru_insert_with_evict() {
        let evict_count = Arc::new(AtomicUsize::default());
        let in_closure = evict_count.clone();
        let mut lru = LruCache::<u8>::with_evict(10, move |_| {
            in_closure.fetch_add(1, Ordering::SeqCst);
        });
        for i in 0_u8..100 {
            lru.insert(i);
        }
        assert_eq!(
            VecDeque::from([99, 98, 97, 96, 95, 94, 93, 92, 91, 90]),
            lru.data
        );
        assert_eq!(90, evict_count.load(Ordering::SeqCst));
    }

    #[test]
    fn test_lru_evict_all_on_drop() {
        let evict_count = Arc::new(AtomicUsize::default());
        let in_closure = evict_count.clone();
        let mut lru = LruCache::<u8>::with_evict(10, move |_| {
            in_closure.fetch_add(1, Ordering::SeqCst);
        });
        for i in 0_u8..10 {
            lru.insert(i);
        }
        drop(lru);

        assert_eq!(10, evict_count.load(Ordering::SeqCst));
    }
}
