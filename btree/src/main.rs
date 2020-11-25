mod node;
mod tree;

use std::fmt::Debug;

use node::Node;

fn main() {
    let mut root = Node::new(12);
    let vec = vec![3, 2, 1, 4, 5, 6, 8,9,12,4,5,22];

    let n = Node::from_vec(vec).unwrap();

    println!("tree: {:?}", n);

    snake_traverse(n);


}

fn snake_traverse<T: Clone + std::cmp::Eq + Debug>(node: Node<T>) {
    let mut result = vec![];
    result.push(node.val.clone());
    let list = vec![node];

    let mut temp = list;
    let mut reverse = true;
    loop {
        temp.iter().for_each(|d| {
            println!("val: {:?}", d.val);
        });
        temp = walk(temp, reverse);
        reverse = !reverse;
        if temp.len() == 0 {
            break;
        }
    }

}

fn walk<T: Clone + std::cmp::Eq>(list: Vec<Node<T>>, reverse: bool) -> Vec<Node<T>> {
    let mut list = list;
    let mut next_list = vec![];
    list.reverse();
    for temp in list.into_iter() {
        if reverse {
            if let Some(left) = &temp.left{
                next_list.push(*left.clone());
            }
            if let Some(right) = &temp.right {
                next_list.push(*right.clone());
            }
        } else {
            if let Some(right) = &temp.right {
                next_list.push(*right.clone());
            }
            if let Some(left) = &temp.left{
                next_list.push(*left.clone());
            }
        }

    }
    next_list
}