
#[derive(Debug)]
pub(crate) struct StackNode<T> {
    pub(crate) data: T,
    pub(crate) next: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
pub(crate) struct Stack<T> {
    pub(crate) top: Option<StackNode<T>>
}

impl<T> StackNode<T> {
    pub(crate) fn new(data: T) -> StackNode<T> {
        StackNode { data, next: None }
    }
}

impl<T> Stack<T> {
    pub(crate) fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub(crate) fn _is_empty(&self) -> bool {
        match self.top {
            Some(_) => false,
            None => true,
        }
    }

    pub(crate) fn push(&mut self, data: T) {
        let mut node = StackNode::new(data);
        if let Some(top) = std::mem::replace(&mut self.top, None) {
            node.next = Some(Box::new(top));
        }
        self.top = Some(node);
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        if let Some(top) = std::mem::replace(&mut self.top, None) {
            self.top = match top.next {
                Some(n) => Some(*n), 
                None => None,
            };
            Some(top.data)
        } else {
            None
        }
    }

    pub(crate) fn top(&self) -> Option<&T> {
        match &self.top {
            Some(top) => Some(&top.data),
            None => None,
        }
    }

    pub(crate) fn next_top(&self) -> Option<&T> {
        match &self.top {
            Some(top) => match &top.next {
                Some(next) => Some(&next.data),
                None => None,
            },
            None => None,
        }
    }
}


#[cfg(test)]
mod stack_test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_new() {
        let mut stack = Stack::<Rc<&str>>::new();
        stack.push(Rc::new("/".into()));
        //println!("stack: {:#?}", stack);
        stack.push(Rc::new("/flight".into()));
        //println!("stack: {:#?}", stack);
        stack.push(Rc::new("/visa".into()));
        //println!("stack: {:#?}", stack);
        let _top = stack.top();
        //println!("top: {:#?}", top);
        //println!("stack: {:#?}", stack);

        let _top_next = stack.next_top();
        //println!("top_next: {:#?}", top_next);
        
        let _pop = stack.pop();
        //println!("pop: {:#?}", pop);
        let _pop2 = stack.pop();
        //println!("pop2: {:#?}", pop2);

        stack.push(Rc::new("/hotel".into()));
        //println!("stack: {:#?}", stack);

    }
}