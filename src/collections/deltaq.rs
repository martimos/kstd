use alloc::collections::VecDeque;
use core::fmt::Debug;
use core::ops::{Index, IndexMut};

/// A queue where every node has a numeric weight.
/// Nodes are stored in the queue sorted by their weight.
/// The weight within the queue is the difference to the weight of the previous node.
///
/// # Example
/// Take a queue with three nodes, weight 1, 2 and 3.
/// When we insert the first node, the queue looks as follows.
///
/// `[1]`
///
/// When we insert the second node, the weight is higher than all nodes in the queue,
/// so it's stored as the last element. The node with which he is stored is the difference
/// from its original weight to the weight of the previous node. The queue looks as follows.
///
/// `[1 (first node), 1 (second node)]`
///
/// Same thing goes for the third node.
///
/// `[1 (first node), 1 (second node), 1 (third node)]`
///
/// No matter the order of insertion, in the end, the queue is always sorted like this.
/// With this insert algorithm, the amount of weight of nodes from the front of the queue to
/// a specific node, is the original weight of that specific node. In the above example, the amount
/// of weights until the third node is 3 (1 + 1 + 1), which is the original weight.
#[derive(Debug, Eq, PartialEq)]
pub struct DeltaQueue<T> {
    data: VecDeque<Node<T>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node<T> {
    pub value: usize,
    pub elem: T,
}

impl<T> Node<T> {
    pub fn new(value: usize, elem: T) -> Self {
        Self { value, elem }
    }
}

impl<T> Default for DeltaQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DeltaQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }

    /// Inserts the given element with the given value into this queue.
    /// Returns a reference to the inserted element.
    pub fn insert(&mut self, value: usize, elem: T) -> &mut T {
        let mut total_value = 0;

        for i in 0..self.data.len() {
            let found = &self.data[i];

            if total_value + found.value >= value {
                // insert at this position
                let node_value = value - total_value;
                self.data.insert(i, Node::new(node_value, elem));

                if self.len() > i + 1 {
                    // if there is a next element, adapt its value
                    self.data[i + 1].value -= node_value;
                }
                return &mut self.data[i].elem;
            }

            total_value += found.value;
        }
        self.data.push_back(Node::new(value - total_value, elem));
        &mut self.data.back_mut().unwrap().elem
    }

    pub fn front(&self) -> Option<&Node<T>> {
        self.data.front()
    }

    pub fn front_mut(&mut self) -> Option<&mut Node<T>> {
        self.data.front_mut()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front().map(|n| n.elem)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> Index<usize> for DeltaQueue<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T> IndexMut<usize> for DeltaQueue<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn test_simple_insert() {
        let mut q = DeltaQueue::new();
        assert_eq!(0, q.len());

        q.insert(0, "hello");
        assert_eq!(1, q.len());
    }

    #[test]
    fn test_insert_in_order() {
        let mut q = DeltaQueue::new();

        q.insert(3, "three");
        q.insert(5, "five");
        q.insert(9, "nine");

        assert_eq!(
            vec![
                Node::new(3, "three"),
                Node::new(2, "five"),
                Node::new(4, "nine"),
            ],
            Vec::from(q.data)
        );
    }

    #[test]
    fn test_insert_reverse_order() {
        let mut q = DeltaQueue::new();

        q.insert(9, "nine");
        q.insert(5, "five");
        q.insert(3, "three");

        assert_eq!(
            vec![
                Node::new(3, "three"),
                Node::new(2, "five"),
                Node::new(4, "nine"),
            ],
            Vec::from(q.data)
        );
    }

    #[test]
    fn test_insert_no_order() {
        let mut q = DeltaQueue::new();

        q.insert(3, "three");
        q.insert(6, "six");
        q.insert(5, "five");
        q.insert(9, "nine");
        q.insert(4, "four");

        assert_eq!(
            vec![
                Node::new(3, "three"),
                Node::new(1, "four"),
                Node::new(1, "five"),
                Node::new(1, "six"),
                Node::new(3, "nine"),
            ],
            Vec::from(q.data)
        );
    }

    #[test]
    fn test_insert_duplicates() {
        let mut q = DeltaQueue::new();

        q.insert(3, "three");
        q.insert(5, "five");
        q.insert(5, "five");
        q.insert(5, "five");
        q.insert(6, "six");
        q.insert(9, "nine");

        assert_eq!(
            vec![
                Node::new(3, "three"),
                Node::new(2, "five"),
                Node::new(0, "five"),
                Node::new(0, "five"),
                Node::new(1, "six"),
                Node::new(3, "nine"),
            ],
            Vec::from(q.data)
        );
    }

    #[test]
    fn test_insert_duplicates_different_values() {
        let mut q = DeltaQueue::new();

        q.insert(5, "five1");
        q.insert(5, "five2");
        q.insert(5, "five3");

        assert_eq!(
            vec![
                Node::new(5, "five3"),
                Node::new(0, "five2"),
                Node::new(0, "five1"),
            ],
            Vec::from(q.data)
        );
    }

    #[test]
    fn test_insert_many() {
        let limit = 1000;
        let mut q = DeltaQueue::with_capacity(limit);

        for i in (0..limit).rev() {
            q.insert(i + 1, i);
            assert_eq!(limit - i, q.len());
        }

        for i in 0..limit {
            let front = &q.data[0];
            assert_eq!(1, front.value);
            assert_eq!(i, front.elem);
            assert_eq!(Some(i), q.pop_front());
        }
    }
}
