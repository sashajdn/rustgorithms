use std::cmp::min;

struct MinStack<T>
where
    T: Ord + PartialEq + Clone,
{
    stack: Vec<T>,
    min_stack: Vec<T>,
}

trait Stack<T>
where
    T: PartialOrd + PartialEq,
{
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn top(&mut self) -> Option<&T>;
}

impl<T> MinStack<T>
where
    T: Ord + PartialEq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        MinStack {
            stack: Vec::with_capacity(capacity),
            min_stack: Vec::with_capacity(capacity),
        }
    }

    pub fn get_min(&self) -> Option<&T> {
        self.min_stack.last()
    }
}

impl<T> Stack<T> for MinStack<T>
where
    T: Ord + PartialEq + Clone,
{
    fn push(&mut self, value: T) {
        self.stack.push(value.clone());

        let min_value = match self.min_stack.last() {
            Some(last_value) => min(last_value, &value),
            None => &value,
        };

        self.min_stack.push(min_value.clone());
    }

    fn pop(&mut self) -> Option<T> {
        let last = self.stack.pop();
        _ = self.min_stack.pop();

        last
    }

    fn top(&mut self) -> Option<&T> {
        self.stack.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_stack_sanity_checks() {
        let mut min_stack = MinStack::new(16);
        assert_eq!(min_stack.top(), None);
        assert_eq!(min_stack.pop(), None);

        min_stack.push(3);
        min_stack.push(1);
        min_stack.push(2);

        assert_eq!(min_stack.top(), Some(2).as_ref());
        assert_eq!(min_stack.get_min(), Some(1).as_ref());

        assert_eq!(min_stack.pop(), Some(2));
        assert_eq!(min_stack.get_min(), Some(1).as_ref());

        assert_eq!(min_stack.pop(), Some(1));
        assert_eq!(min_stack.get_min(), Some(3).as_ref());

        assert_eq!(min_stack.pop(), Some(3));
        assert_eq!(min_stack.get_min(), None);
    }
}
