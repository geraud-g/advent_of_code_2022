#[derive(Debug, Default)]
pub struct ArenaTree<T>
    where
        T: PartialEq
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
    where
        T: PartialEq
{
    pub fn insert_node(&mut self, val: T, parent: Option<usize>) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val, parent));
        if let Some(parent_idx) = parent {
            self.arena.get_mut(parent_idx).unwrap().children.push(idx);
        }
        idx
    }

    pub fn get_unwrapped(&self, idx: usize) -> &Node<T> {
        self.arena.get(idx).unwrap()
    }
}


#[derive(Debug)]
pub struct Node<T>
    where
        T: PartialEq
{
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}


impl<T> Node<T>
    where
        T: PartialEq
{
    fn new(idx: usize, val: T, parent: Option<usize>) -> Self {
        Self {
            idx,
            val,
            parent,
            children: vec![],
        }
    }
}
