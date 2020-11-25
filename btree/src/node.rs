use std::fmt::Debug;
use std::fmt;

#[derive( Clone)]
pub struct Node<T: Clone + PartialEq+ Eq> {
  pub left: Option<Box<Node<T>>>,
  pub right: Option<Box<Node<T>>>,
  pub val: T,
}

impl<T: Debug + PartialEq + Eq + Clone> fmt::Debug for Node<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("V:{:?} L:{:?} R:{:?}", self.val, self.left, self.right))
  }
}

impl<T: Clone + PartialEq + Eq + std::cmp::PartialOrd + Debug> Node<T> {
  pub fn new(val: T) -> Self{
    Self {
      left: None,
      right: None,
      val,
    }
  }

  pub fn from_vec(vec: Vec<T>) -> Result<Self, String> {
    if vec.is_empty() {
      return Err("vec is empty".to_string());
    }
    let mut count = 0;
    let mut root = Node::new(vec[count].clone());
    while count < vec.len() - 1 {
      count = count + 1;
      let new_val = vec[count].clone();
      root.insert(new_val);
    }
    Ok(root)
  }

  pub fn insert(&mut self,val: T) {
    // 新值大于老值，放右边，和右边比较
    if self.val >= val {
      match &mut self.left {
        Some(left) => {
          left.insert(val.clone());
        },
        None => {
          self.set_left(Node::new(val.clone()));
        }
      }
      return;
    }

    match &mut self.right {
      Some(right) => {
        right.insert(val.clone());
      },
      None => {
        self.set_right(Node::new(val.clone()));
      }
    }
  }

  pub fn set_left(&mut self, node: Node<T>) {
    self.left = Some(Box::new(node));
  }

  pub fn set_right(&mut self, node: Node<T>) {
    self.right = Some(Box::new(node));
  }
}
