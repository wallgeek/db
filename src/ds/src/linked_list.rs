struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

pub struct LinkedList <T> {
    head: Option<Box<Node<T>>>
}

impl<T: Eq + Copy> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn add(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take()
        }))
    }

    pub fn remove(&mut self, value: T) {
        let head_mut = self.head.as_mut();
        
        // if head matches
        if let Some(mut head_node) = head_mut {
            if head_node.value == value {
                self.head = head_node.next.take();

                return;
            }else {
                while head_node.next.is_some() {
                    let next =  head_node.next.take();
                    let mut next_node = next.unwrap();

                    if next_node.value == value {
                        let next_next = next_node.as_mut().next.take();
                        head_node.next = next_next;
                        break;
                    }else {
                        head_node.next = Some(next_node);
                    }

                    head_node = head_node.next.as_mut().unwrap();
                }
            }
        }
    }

    pub fn collect(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        let mut curr_ref = self.head.as_ref();//.unwrap();

        loop {
            if curr_ref.is_none() {
                break;
            }

            let node = curr_ref.unwrap();
            
            result.push(node.value);
            
            curr_ref = node.next.as_ref();
        }

        result
    }

    pub fn is_empty(&self) -> bool {
        if self.head.is_none() {
            true
        }else {
            false
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(mut boxed_node) = head {
            head = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn add() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        let mut values: Vec<u8> = Vec::new();
        let random_len = rand::thread_rng().gen_range(0..100) as usize;
        
        for _ in 0..random_len {
            values.push(rand::thread_rng().gen_range(0..255))
        }

        for index in 0..random_len {
            ll.add(values[index]);
        }

        let ll_to_vec = ll.collect();

        assert_eq!(ll_to_vec.len(), random_len);

        for index in 0..random_len {
            assert_eq!(ll_to_vec[random_len - index - 1], values[index]);
        }
    }

    #[test]
    fn remove(){
        let mut ll: LinkedList<u8> = LinkedList::new();
        let first: u8 = 1;
        let middle: u8 = 128;
        let last: u8 = 255;
        let values: Vec<u8> = vec![first, 10, 10, 10, middle, 10, 10, 10, last];
        
        for index in 0..values.len() {
            ll.add(values[index]);
        }

        ll.remove(first);
        ll.remove(middle);
        ll.remove(last);
        
        let to_vec = ll.collect();

        assert_eq!(to_vec.len(), values.len() - 3);
        
        for value in to_vec{
            assert_eq!(value, 10)
        }
    }
}