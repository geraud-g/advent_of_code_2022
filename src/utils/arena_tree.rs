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
    fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
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
