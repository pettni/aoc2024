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
}
