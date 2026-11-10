use std::collections::{BTreeSet, HashSet, VecDeque};

trait Container<T>: From<Vec<T>> {
    fn pop(&mut self) -> Option<T>;
    fn empty(&self) -> bool;
    fn insert(&mut self, item: T);
}

pub struct Queue<T> {
    inner: VecDeque<T>,
}

impl<T> Container<T> for Queue<T> {
    fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    fn insert(&mut self, item: T) {
        self.inner.push_back(item);
    }

    fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> From<Vec<T>> for Queue<T> {
    fn from(vec: Vec<T>) -> Self {
        Queue {
            inner: VecDeque::from(vec),
        }
    }
}

pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Container<T> for Stack<T> {
    fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn insert(&mut self, item: T) {
        self.inner.push(item);
    }

    fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(vec: Vec<T>) -> Self {
        Stack { inner: vec }
    }
}

pub struct Node<T> {
    value: T,
    neighbors: BTreeSet<usize>,
}

pub trait Graph<T> {
    fn nodes(&self) -> &[Node<T>];
}

#[inline(always)]
pub fn dfs<T: PartialEq, G: Graph<T>>(g: G, to: T, from: T) -> bool {
    cfs::<T, Stack<usize>, G>(g, to, from)
}

#[inline(always)]
pub fn bfs<T: PartialEq, G: Graph<T>>(g: G, to: T, from: T) -> bool {
    cfs::<T, Queue<usize>, G>(g, to, from)
}

fn cfs<T: PartialEq, C: Container<usize>, G: Graph<T>>(g: G, to: T, from: T) -> bool {
    let nodes = g.nodes();
    let Some(start) = nodes.iter().position(|node| node.value == from) else {
        return false;
    };

    let mut visited = HashSet::new();
    let mut container = C::from(vec![start]);

    while !container.empty() {
        let Some(next) = container.pop() else {
            break;
        };

        if !visited.insert(next) {
            continue;
        }

        let Some(node) = nodes.get(next) else {
            continue;
        };
        if node.value == to {
            return true;
        }

        for &neighbor in &node.neighbors {
            container.insert(neighbor);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{bfs, dfs, Graph, Node};
    use std::collections::BTreeSet;

    struct TestGraph {
        nodes: Vec<Node<u32>>,
    }

    impl Graph<u32> for TestGraph {
        fn nodes(&self) -> &[Node<u32>] {
            &self.nodes
        }
    }

    fn an_arbitrary_graph_with_100_values() -> TestGraph {
        // A directed chain visits every value in 0..100 from the first node.
        let nodes = (0u32..100)
            .map(|value| Node {
                value,
                neighbors: if value < 99 {
                    BTreeSet::from([(value + 1) as usize])
                } else {
                    BTreeSet::new()
                },
            })
            .collect();
        TestGraph { nodes }
    }

    #[test]
    fn correctly_find_value_via_dfs() {
        let graph = an_arbitrary_graph_with_100_values();
        assert_eq!(graph.nodes().len(), 100);
        assert!(dfs(graph, 42, 0));
    }

    #[test]
    fn correctly_find_value_via_bfs() {
        let graph = an_arbitrary_graph_with_100_values();
        assert_eq!(graph.nodes().len(), 100);
        assert!(bfs(graph, 99, 0));
    }

    #[test]
    fn does_not_find_value_outside_the_graph() {
        assert!(!dfs(an_arbitrary_graph_with_100_values(), 100, 0));
        assert!(!bfs(an_arbitrary_graph_with_100_values(), 100, 0));
    }
}
