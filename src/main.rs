#![allow(dead_code)]
mod algorithms;
mod thread;

use algorithms::data_structure::tree::binary_tree::Tree;
fn main() {
    let mut tree = Tree::new(5);

    tree.push(1);
    tree.push(2);
    tree.push(6);
    tree.push(8);
    tree.push(5);

    tree.pre_order_recursive();

    println!("");
    println!("");

    tree.pre_order_loop();

    println!("");
    println!("");
}
