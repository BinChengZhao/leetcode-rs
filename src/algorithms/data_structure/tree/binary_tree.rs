//start 15.25
//start 16.19

#[derive(Debug, Default)]
pub struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type Root = Option<Box<TreeNode>>;

pub struct Tree {
    root: Root,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Tree {
    pub fn new(val: i32) -> Tree {
        let root = Some(Box::new(TreeNode::new(val)));

        Tree { root }
    }

    pub fn push(&mut self, val: i32) {
        let tree_node = Some(Box::new(TreeNode::new(val)));
        let mut root = &mut self.root;

        while let Some(root_node) = root {
            let root_val = root_node.val;

            if val > root_val {
                root = &mut root_node.right;
            } else {
                root = &mut root_node.left;
            }
        }

        *root = tree_node;
    }

    // Maybe it is BST.
    pub fn pre_order_loop(&self) {
        let mut buffer: Vec<&Root> = Vec::new();
        buffer.push(&self.root);

        while let Some(option_node) = buffer.pop() {
            if let Some(root_node) = option_node {
                println!("{}", root_node.val);
                buffer.push(&root_node.right);
                buffer.push(&root_node.left);
            }
        }
    }

    pub fn pre_order_recursive(&self) {
        Self::pre_order_recursive_inner(&self.root);
    }

    fn pre_order_recursive_inner(root: &Root) {
        if let Some(root_node) = root {
            println!("{}", root_node.val);
            Self::pre_order_recursive_inner(&root_node.left);
            Self::pre_order_recursive_inner(&root_node.right);
        }
    }
}

impl Drop for TreeNode {
    fn drop(&mut self) {
        println!("{}", self.val);
    }
}
