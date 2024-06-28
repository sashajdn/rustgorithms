use std::cmp::{Ord, Ordering};

pub struct Heap<C, T>
where
    C: Fn(&T, &T) -> Ordering,
{
    data: Vec<T>,
    comparator: C,
}

pub trait Heaper<T> {
    fn pop(&mut self) -> Option<T>;

    fn insert(&mut self, v: T);

    fn peek(&self) -> Option<&T>;
}

impl<C, T> Heap<C, T>
where
    C: Fn(&T, &T) -> Ordering,
    T: Ord,
{
    pub fn new(comparator: C, capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            comparator,
        }
    }
}

impl<C, T> Heaper<T> for Heap<C, T>
where
    C: Fn(&T, &T) -> Ordering,
    T: Ord,
{
    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);

        let result = self.data.pop();

        let mut parent_index = 0;
        let mut largest = parent_index;
        loop {
            let left_child_index = 2 * parent_index + 1;
            let right_child_index = 2 * parent_index + 2;

            if left_child_index < self.data.len()
                && (self.comparator)(&self.data[left_child_index], &self.data[parent_index])
                    == Ordering::Less
            {
                largest = left_child_index;
            }

            if right_child_index < self.data.len()
                && (self.comparator)(&self.data[right_child_index], &self.data[largest])
                    == Ordering::Less
            {
                largest = right_child_index;
            }

            if largest != parent_index {
                self.data.swap(largest, parent_index);
                parent_index = largest;
                continue;
            }

            break;
        }

        result
    }

    fn insert(&mut self, v: T) {
        self.data.push(v);
        let mut child_index = self.data.len() - 1;
        if child_index == 0 {
            return;
        }

        let mut parent_index = (child_index - 1) / 2;
        while child_index > 0
            && (self.comparator)(&self.data[child_index], &self.data[parent_index])
                == Ordering::Less
        {
            self.data.swap(child_index, parent_index);
            child_index = parent_index;
            if child_index == 0 {
                break;
            }

            parent_index = (child_index - 1) / 2;
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_heap() {
        let min = |a: &i32, b: &i32| a.cmp(b);
        let mut min_heap = Heap::new(min, 10);

        min_heap.insert(3);
        min_heap.insert(4);
        min_heap.insert(5);
        min_heap.insert(10);
        min_heap.insert(9);
        min_heap.insert(7);

        assert_eq!(min_heap.peek(), Some(&3));
        assert_eq!(min_heap.pop(), Some(3));
        assert_eq!(min_heap.peek(), Some(&4));

        min_heap.insert(1);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.peek(), Some(&4));
        assert_eq!(min_heap.pop(), Some(4));
        assert_eq!(min_heap.pop(), Some(5));
    }
}
