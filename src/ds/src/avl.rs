//         y                           x
//        / \                         / \   
//       /   \     Right Rotate      /   \
//      x     T3 ---------------->  T1    y
//     / \       <----------------       / \
//    /   \        Left Rotate          /   \
//  T1     T2                         T2     T3
// T will only be implementable for Numbers type only
pub trait RationalNumber: Copy + PartialOrd {}

impl RationalNumber for u32 {}
impl RationalNumber for i32 {}
impl RationalNumber for u64 {}
impl RationalNumber for i64 {}
impl RationalNumber for f32 {}
impl RationalNumber for f64 {}
impl RationalNumber for usize {}
impl RationalNumber for isize {}

// Define Node
#[derive(Debug)]
struct Node<T> {
    value: T,
    height: u8,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T: RationalNumber> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            height: 1,
            left: None,
            right: None
        }
    }

    fn copy(&mut self) -> Box<Node<T>> {
        let mut copy_node = Box::new(Node::new(self.value));
        
        copy_node.height = self.height;
        copy_node.left = self.left.take();
        copy_node.right = self.right.take();
        
        copy_node  
    }

    fn update_height(&mut self) {
        let left = &self.left;
        let right = &self.right;
        let mut left_height = 0;
        let mut right_height = 0;

        match left {
            None => {},
            Some(node) => left_height = node.height
        }

        match right {
            None => {},
            Some(node) => right_height = node.height
        }

        self.height = 1 + std::cmp::max(left_height, right_height);
    }

    fn get_balance_factor(&self) -> i8 {
        let left = &self.left;
        let right = &self.right;

        let mut left_height: u8 = 0;
        let mut right_height: u8 = 0;

        match left {
            None => {},
            Some(node) => {
                left_height = node.height;
            }
        }

        match right {
            None => {},
            Some(node) => {
                right_height = node.height;
            }
        }

        (left_height as i8) - (right_height as i8)
    }

    fn left_rotate(&mut self) -> Box<Node<T>> {
        let mut y_node = self.right.take().unwrap();
        let t1: Option<Box<Node<T>>> = self.left.take();
        let t2: Option<Box<Node<T>>> = y_node.left.take();
        
        let mut x_node = self.copy();

        x_node.left = t1;
        x_node.right = t2;
        x_node.update_height();
        
        y_node.left = Some(x_node);
        y_node.update_height();
        
        return y_node       
    }

    fn right_rotate(&mut self) -> Box<Node<T>> {
        let mut x_node = self.left.take().unwrap();
        let t3: Option<Box<Node<T>>> = self.right.take();
        let t2: Option<Box<Node<T>>> = x_node.right.take();
        
        let mut y_node = self.copy();

        y_node.left = t2;
        y_node.right = t3;
        y_node.update_height();
        
        x_node.right = Some(y_node);
        x_node.update_height();
        
        return x_node        
    }

    fn balance_after_add(&mut self, value: T) -> Box<Node<T>>{
        self.update_height();

        let bf = self.get_balance_factor();
        
        if bf == -1 || bf == 0 || bf == 1 {
            return self.copy();
        }else if bf > 1 {
            let node = self.left.as_mut().unwrap();

            if value > node.value {
                self.left = Some(node.left_rotate());
            }

            return self.right_rotate();
        }else {
            let node = self.right.as_mut().unwrap();

            if value < node.value {
                self.right = Some(node.right_rotate());
            }

            return self.left_rotate();
        }
    }

    fn balance_after_remove(&mut self) -> Box<Node<T>> {
        self.update_height();

        let bf = self.get_balance_factor();

        if bf == -1 || bf == 0 || bf == 1 {
            return self.copy();
        }else if bf > 1 {
            let left = &mut self.left;
            
            match left{
                None => {},
                Some(left_node) => {
                    let left_node_bf = left_node.get_balance_factor();

                    if left_node_bf < 0 {
                        self.left = Some(left_node.left_rotate());
                    }
                }
            }
            
            return self.right_rotate();
        }else {
            let right = &mut self.right;
            
            match right {
                None => {},
                Some(right_node) => {
                    let right_node_bf = right_node.get_balance_factor();

                    if right_node_bf > 0 {
                        self.right = Some(right_node.right_rotate());
                    }
                }
            }
            
            return self.left_rotate();
        }        
    }

    fn get_leftmost_child(&self) -> &Node<T> {
        let left = &self.left;

        match left {
            None => {
                return self
            },
            Some(node) => {
                return node.get_leftmost_child()
            }
        }
    }

    pub fn add(&mut self, value: T) -> Option<Box<Node<T>>> {
        let curr_val = &self.value;

        if curr_val > &value {
            let left = &mut self.left;
            match left {
                None => {
                    self.left = Some(Box::new(Node::new(value)))        
                },
                Some(n) => {
                    self.left = n.add(value);
                }
            }
            
        }else if curr_val < &value {
            let right = &mut self.right;
            match right {
                None => {
                    self.right = Some(Box::new(Node::new(value)))        
                },
                Some(n) => {
                    self.right = n.add(value);
                }
            }
        }
        
        Some(self.balance_after_add(value))
    }

    pub fn remove(&mut self, value: T) -> Option<Box<Node<T>>> {
        if value < self.value {
            let left = self.left.take();

            match left {
                None => {
                    self.left = left;
                    return Some(self.copy())
                },
                Some(mut left_node) => {
                    self.left = left_node.remove(value)
                }
            }
        }else if value > self.value {
            let right = self.right.take();

            match right {
                None => {
                    self.right = right;
                    return Some(self.copy())
                },
                Some(mut right_node) => {
                    self.right = right_node.remove(value)
                }
            }
        }else if self.value == value {
            // find the childs
            let left = self.left.take();
            let right = self.right.take();

            if left.is_none() == true && right.is_none() == true {
                return None
            }else if left.is_none() == true {
                return right
            }else if right.is_none() == true {
                return left
            }else {
                let mut right_node = right.unwrap();
                let highest_node = right_node.get_leftmost_child();
                let highest_value = highest_node.value;

                self.value = highest_value;
                self.left = left;
                self.right = right_node.remove(highest_value);
            }
        }else {
            return Some(self.copy())
        }
        
        Some(self.balance_after_remove())
    }
}

#[derive(Debug)]
pub struct AVL<T> {
    root: Option<Box<Node<T>>>
}

impl<T: RationalNumber> AVL<T> {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn add(&mut self, value: T) {
        let some_node = self.root.as_mut();
        
        if let Some(node) = some_node {
            self.root = node.add(value);
        }else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }

    pub fn remove(&mut self, value: T) {

        let some_node = self.root.as_mut();
        
        if let Some(node) = some_node {
            self.root = node.remove(value);
        }
    }

    pub fn get_root(&self) -> Option<&T> {
        let some_node = self.root.as_ref();

        if let Some(node) = some_node {
            Some(&node.value)
        }else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        let some_node = self.root.as_ref();

        if some_node.is_some() {
            false
        }else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let nodes: Vec<i32> = vec![21,26,30,9,4,14,28,18,15,10,2,3,7];
        let mut root: Option<Box<Node<i32>>> = Some(Box::new(Node::new(nodes[0])));
        
        for index in 1..nodes.len() {
            let val = nodes[index];

            root = root.unwrap().add(val);
        }
            
        assert_eq!(root.unwrap().value, 14);
    }

    #[test]
    fn remove() {
        let nodes: Vec<i32> = vec![21,26,30,9,4,14,28,18,15,10,2,3,7];
        let mut root: Option<Box<Node<i32>>> = Some(Box::new(Node::new(nodes[0])));
        
        for index in 1..nodes.len() {
            let val = nodes[index];

            root = root.unwrap().add(val);
        }

        root = root.unwrap().remove(14);
        root = root.unwrap().remove(18);
        root = root.unwrap().remove(21);
        root = root.unwrap().remove(26);
        root = root.unwrap().remove(28);
        root = root.unwrap().remove(2);
        root = root.unwrap().remove(9);
        root = root.unwrap().remove(10);
        root = root.unwrap().remove(15);
        
        assert_eq!(root.unwrap().value, 4);
    }
}
