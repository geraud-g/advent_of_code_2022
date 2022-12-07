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
    pub fn insert_node(&mut self, val: T) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
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
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}
