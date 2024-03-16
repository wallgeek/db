pub struct Stack<Type>(Vec<Type>);

impl<Type> Stack<Type> { 
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        if self.0.len() == 0 {
            true
        }else {
            false
        }
    }

    pub fn add(&mut self, value: Type) {
        self.0.push(value);
    }

    pub fn take(&mut self) -> Option<Type> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&Type> {
        let len = self.0.len();

        if len == 0 {
            None
        }else {
            Some(&self.0[len - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_peek() {
        let mut stack: Stack<u32> = Stack::new();
        
        let value: u32 = 1;
        
        stack.add(value);

        let last_element  = stack.peek().unwrap();

        assert_eq!(&value, last_element);

        // add more and test
        let value: u32 = 2;

        stack.add(value);

        let last_element  = stack.peek().unwrap();

        assert_eq!(&value, last_element);
    }

    #[test]
    fn take_is_empty() {
        let mut stack: Stack<u32> = Stack::new();
        let value1: u32 = 1;
        let value2: u32 = 2;

        stack.add(value1);
        stack.add(value2);
        stack.take();

        let last_element  = stack.peek().unwrap();

        assert_eq!(&value1, last_element);
        assert!(!stack.is_empty());
        stack.take();
        assert!(stack.is_empty());
    }
}
