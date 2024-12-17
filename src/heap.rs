use std::cmp::Ordering;

pub struct MinHeap<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    data: Vec<T>,
    cmp: F,
}

impl<T, F> MinHeap<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    pub fn new(cmp: F) -> Self {
        MinHeap { data: vec![], cmp }
    }

    pub fn with_capacity(capacity: usize, cmp: F) -> Self {
        MinHeap {
            data: Vec::with_capacity(capacity),
            cmp,
        }
    }

    pub fn push(&mut self, t: T) {
        self.data.push(t);
        self.bubble_up();
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let n = self.data.len();
        self.data.swap(0, n - 1);
        let ret = self.data.pop();
        self.push_down();
        ret
    }

    pub fn replace(&mut self, t: T) -> Option<T>
    where
        T: Copy,
    {
        if self.data.is_empty() {
            return None;
        }
        let ret = self.data[0];
        self.data[0] = t;
        self.push_down();
        Some(ret)
    }

    fn bubble_up(&mut self) {
        // bubble up last element to preserve heap structure
        // indexing: parent of n is (n-1)/2
        let mut n = self.data.len() - 1; // index to bubble up
        while n >= 1 && (self.cmp)(&self.data[(n - 1) / 2], &self.data[n]) == Ordering::Greater {
            let n_parent = (n - 1) >> 1;
            self.data.swap(n, n_parent);
            n = n_parent;
        }
    }

    fn push_down(&mut self) {
        // push down first element to preserve heap structure
        // indexing: children of n are 2n+1, 2n+2
        let mut n = 0;
        while 2 * n + 1 < self.data.len() {
            // while not child
            let n1 = 2 * n + 1;
            let n2 = 2 * n + 2;

            // find child with smallest value
            let n_child = if n1 == self.data.len() - 1 {
                n1 // just one child node
            } else {
                match (self.cmp)(&self.data[n1], &self.data[n2]) {
                    Ordering::Less | Ordering::Equal => n1,
                    Ordering::Greater => n2,
                }
            };

            // compare parent to smallest child
            if (self.cmp)(&self.data[n], &self.data[n_child]) == Ordering::Greater {
                self.data.swap(n, n_child);
                n = n_child;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_up() {
        let data = vec![0, /**/ 1, 2, /**/ 3, 4, 5, 6, /**/ 7];
        let mut heap = MinHeap {
            data,
            cmp: |x, y| x.cmp(y),
        };

        heap.data.push(-1);
        heap.bubble_up();

        assert_eq!(heap.data, vec![-1, 0, 2, 1, 4, 5, 6, 7, 3]);

        heap.data.push(-2);
        heap.bubble_up();

        assert_eq!(heap.data, vec![-2, -1, 2, 1, 0, 5, 6, 7, 3, 4]);

        heap.data.push(-3);
        heap.bubble_up();

        assert_eq!(heap.data, vec![-3, -2, 2, 1, -1, 5, 6, 7, 3, 4, 0]);

        heap.data.push(-4);
        heap.bubble_up();

        assert_eq!(heap.data, vec![-4, -2, -3, 1, -1, 2, 6, 7, 3, 4, 0, 5]);
    }

    #[test]
    fn test_push_down() {
        let data = vec![0, /**/ 1, 2, /**/ 3, 4, 5, 6, /**/ 7];
        let mut heap = MinHeap {
            data,
            cmp: |x, y| x.cmp(y),
        };

        heap.data[0] = 8;
        heap.push_down();

        assert_eq!(heap.data, vec![1, /**/ 3, 2, /**/ 7, 4, 5, 6, /**/ 8]);

        heap.data[0] = 9;
        heap.push_down();

        assert_eq!(heap.data, vec![2, /**/ 3, 5, /**/ 7, 4, 9, 6, /**/ 8]);

        heap.data[0] = 10;
        heap.push_down();

        assert_eq!(heap.data, vec![3, /**/ 4, 5, /**/ 7, 10, 9, 6, /**/ 8]);

        heap.push(11);
        heap.push(12);
        heap.push(13);
        heap.push(14);

        heap.data[0] = 15;
        heap.push_down();

        assert_eq!(
            heap.data,
            vec![4, /**/ 7, 5, /**/ 8, 10, 9, 6, /**/ 15, 11, 12, 13, 14]
        );

        heap.data[0] = 16;
        heap.push_down();

        assert_eq!(
            heap.data,
            vec![5, /**/ 7, 6, /**/ 8, 10, 9, 16, /**/ 15, 11, 12, 13, 14]
        );

        heap.data[0] = 17;
        heap.push_down();

        assert_eq!(
            heap.data,
            vec![6, /**/ 7, 9, /**/ 8, 10, 14, 16, /**/ 15, 11, 12, 13, 17]
        );
    }

    #[test]
    fn test_heap() {
        let mut heap = MinHeap::with_capacity(3, |x: &i64, y: &i64| x.cmp(y));
        assert_eq!(heap.pop(), None);

        heap.push(5);
        heap.push(3);
        heap.push(4);
        heap.push(1);
        heap.push(2);

        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.peek(), Some(&2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.peek(), Some(&3));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.peek(), Some(&4));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.peek(), Some(&5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_replace() {
        let mut heap = MinHeap::with_capacity(3, |x: &i64, y: &i64| x.cmp(y));

        heap.push(5);
        heap.push(3);
        heap.push(4);
        heap.push(1);
        heap.push(2);

        let x = heap.replace(-1);
        assert_eq!(x, Some(1));
        assert_eq!(heap.peek(), Some(&-1));

        let x = heap.replace(4);
        assert_eq!(x, Some(-1));

        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }
}
