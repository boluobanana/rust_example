mod node;
mod tree;

use node::Node;

fn main() {
    let mut root = Node::new(12);
    let vec = vec![3, 2, 1, 4, 5];

    let n = Node::from_vec(vec);

    println!("tree: {:?}", n);
}
