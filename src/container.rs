pub struct StaticStack<T, const CAPACITY: usize> {
    data: [T; CAPACITY],
    ptr: usize,
}

impl<T: Copy + Default, const CAPACITY: usize> StaticStack<T, CAPACITY> {
    pub fn new() -> Self {
        StaticStack {
            data: [T::default(); CAPACITY],
            ptr: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.ptr
    }

    pub fn push(&mut self, element: T) {
        if self.ptr >= CAPACITY {
            panic!("Out of stack");
        }
        self.data[self.ptr] = element;
        self.ptr += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let ret = self.data[self.ptr - 1];
        self.ptr -= 1;
        Some(ret)
    }
}

impl<T: Copy + Default, const CAPACITY: usize> Default for StaticStack<T, CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of the Priority Queue concept for the special case of integer-valued
/// costs with bounded increases.
///
/// https://en.wikipedia.org/wiki/Bucket_queue
pub struct BucketQueue<T> {
    buckets: Vec<Vec<T>>,
    priority: u64,
}

impl<T> BucketQueue<T> {
    /// Create a new BucketQueue.
    ///
    /// The number of buckets num_buckets bounds the largest
    /// priority increment between current node and newly inserted node.
    ///
    /// In particular, a bucket queue with N buckets allows cost increments in [0, N-1).
    pub fn new(num_buckets: usize) -> Self
    where
        T: Clone,
    {
        let mut buckets: Vec<Vec<T>> = vec![];
        buckets.resize(num_buckets, vec![]);
        BucketQueue {
            buckets,
            priority: 0,
        }
    }

    /// Check if queue is empty.
    pub fn is_empty(&self) -> bool {
        self.buckets.iter().all(|x| x.is_empty())
    }

    /// Return length of bucket.
    pub fn len(&self) -> usize {
        self.buckets.iter().map(|x| x.len()).sum()
    }

    /// Remove all elements and reset priority counter to 0.
    pub fn reset(&mut self) {
        for bucket in self.buckets.iter_mut() {
            bucket.clear();
        }
        self.priority = 0;
    }

    /// Push a new priority-val pair to the queue.
    ///
    /// The priority must be s.t. internal_priority <= priority < internal_priority + n,
    /// where n is the number of buckets.
    pub fn push(&mut self, priority: u64, val: T) {
        assert!(self.priority <= priority);
        assert!(priority < self.priority + self.buckets.len() as u64);
        let bucket_id = priority as usize % self.buckets.len();
        self.buckets[bucket_id].push(val);
    }

    /// Return and return an element with the lowest priority.
    ///
    /// Returns (priority, element) if an element is found.
    ///
    /// Increments the internal priority counter until an element is found.
    /// Future elements must have a priority at least as large as the most
    /// recently returned priority.
    pub fn pop(&mut self) -> Option<(u64, T)> {
        for _ in 0..self.buckets.len() {
            if !self.buckets[self.priority as usize % self.buckets.len()].is_empty() {
                break;
            }
            self.priority += 1;
        }
        let bucket_id = self.priority as usize % self.buckets.len();
        self.buckets[bucket_id].pop().map(|x| (self.priority, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = StaticStack::<u32, 32>::new();
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_bucket_queue() {
        let mut queue = BucketQueue::<()>::new(5);
        queue.push(1, ());
        queue.push(2, ());
        queue.push(0, ());

        assert_eq!(queue.len(), 3);

        let (cur_prio, _) = queue.pop().unwrap();
        assert_eq!(cur_prio, 0);
        assert_eq!(queue.priority, 0);

        let (cur_prio, _) = queue.pop().unwrap();
        assert_eq!(cur_prio, 1);
        assert_eq!(queue.priority, 1);

        queue.push(4, ());

        let (cur_prio, _) = queue.pop().unwrap();
        assert_eq!(cur_prio, 2);
        assert_eq!(queue.priority, 2);

        queue.push(5, ());

        let (cur_prio, _) = queue.pop().unwrap();
        assert_eq!(cur_prio, 4);
        assert_eq!(queue.priority, 4);

        queue.push(4, ());
        queue.push(8, ());

        let (cur_prio, _) = queue.pop().unwrap();
        assert_eq!(cur_prio, 4);

        let (cur_prio, _) = queue.pop().unwrap();
        assert_eq!(cur_prio, 5);

        let (cur_prio, _) = queue.pop().unwrap();
        assert_eq!(cur_prio, 8);

        assert!(queue.is_empty());
    }
}
