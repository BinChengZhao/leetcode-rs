#[derive(Debug, Default)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

type Root = Option<Box<Node>>;

#[derive(Debug, Default)]
pub struct LinkedList {
    root: Root,
}

impl Node {
    pub fn new(val: i32) -> Node {
        Node { val, next: None }
    }
}

impl LinkedList {
    pub fn new(val: i32) -> LinkedList {
        let root = Some(Box::new(Node::new(val)));
        LinkedList { root }
    }
    pub fn push(&mut self, val: i32) {
        let mut node = Node::new(val);
        let raw_root = self.root.take();
        node.next = raw_root;
        self.root = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Root {
        let mut raw_root = self.root.take();
        let raw_root_next = raw_root.as_mut().map(|e| e.next.take()).flatten();
        self.root = raw_root_next;

        raw_root
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Bye {}", self.val);
    }
}

// strat at 14.50
// end at 15.20
