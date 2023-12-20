use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::PartialEq;

pub struct LinkedList<T: Copy> {
    size: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

struct Node<T: Copy> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            next: None,
        }
    }

    fn is_last(&self) -> bool {
        return self.next.clone().is_none();
    }
    fn get_next(&self) -> Option<Rc<RefCell<Node<T>>>> {
        return self.next.clone();
    }
    fn get_data(&self) -> T {
        return self.data.clone();
    }
    fn get_ref_data(&self) -> &T {
        return &self.data;
    }
    fn update_next(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        self.next = node;
    }
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }
    fn create_node(value: T) -> Rc<RefCell<Node<T>>> {
        let node = Node::new(value);
        return Rc::new(RefCell::new(node));
    }

    fn check_node_is_equals(left: Option<Rc<RefCell<Node<T>>>>, right: Option<Rc<RefCell<Node<T>>>>) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => Rc::ptr_eq(&left, &right),
            _ => false,
        }
    }

    fn check_node_is_head(&self, node: Option<Rc<RefCell<Node<T>>>>) -> bool {
        return LinkedList::check_node_is_equals(node, self.head.clone());
    }

    fn check_node_is_tail(&self, node: Option<Rc<RefCell<Node<T>>>>) -> bool {
        return LinkedList::check_node_is_equals(node, self.tail.clone());
    }

    fn update_head(&mut self, node: Option<Rc<RefCell<Node<T>>>>)->bool {
        if self.check_node_is_head(node.clone()) {
            self.head = node.unwrap().borrow_mut().get_next();
            return true;
        }
        return false;
    }

    fn update_tail(&mut self, node: Option<Rc<RefCell<Node<T>>>>,prev:Option<Rc<RefCell<Node<T>>>>)->bool{
        if self.check_node_is_tail(node.clone()) {
            match prev {
                None => {
                    self.tail = None;
                }
                Some(prev) => {
                    prev.borrow_mut().update_next(None);
                    self.tail = Some(prev);
                }
            }
            return true;
        }
        return false;
    }
}

pub trait Add {
    type Target;
    fn add_first(&mut self, value: Self::Target);
    fn add_last(&mut self, value: Self::Target);
}

impl<T: Copy> Add for LinkedList<T> {
    type Target = T;

    fn add_first(&mut self, value: Self::Target) {
        let node = LinkedList::create_node(value);
        self.size += 1;

        match self.head.take() {
            None => {
                self.tail = Some(node.clone());
            }
            Some(old_head) => {
                node.borrow_mut().next = Some(old_head);
            }
        }
        self.head = Some(node);
    }

    fn add_last(&mut self, value: Self::Target) {
        let node = LinkedList::create_node(value);
        self.size += 1;

        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
            }
        }
        self.tail = Some(node.clone());
    }
}

pub trait Read {
    type Target: Copy;
    fn get_first(&self) -> Option<Self::Target>;
    fn get_last(&self) -> Option<Self::Target>;
    fn to_array(&self) -> Vec<Self::Target>;
}

impl<T: Copy> Read for LinkedList<T> {
    type Target = T;

    fn get_first(&self) -> Option<Self::Target> {
        if let Some(node) = self.head.clone() {
            return Some(node.borrow().get_data());
        }
        return None;
    }

    fn get_last(&self) -> Option<Self::Target> {
        if let Some(node) = self.tail.clone() {
            return Some(node.borrow().get_data());
        }
        return None;
    }

    fn to_array(&self) -> Vec<Self::Target> {
        let mut vec = Vec::with_capacity(self.size);

        let mut current = self.head.clone();
        while let Some(node) = current {
            vec.push(node.clone().borrow().get_data());


            if let Some(next) = node.clone().borrow().next.clone() {
                current = Some(next.clone());
            } else {
                current = None;
            }
        }

        return vec;
    }
}

pub trait Del {
    type Target: Copy;
    fn del_first(&mut self) -> Option<Self::Target>;
    fn del_last(&mut self) -> Option<Self::Target>;
    fn del_first_value(&mut self, value: Self::Target) -> Option<Self::Target>;
    fn del_last_value(&mut self, value: Self::Target) -> Option<Self::Target>;
}


impl<T: Copy + PartialEq> Del for LinkedList<T> {
    type Target = T;

    fn del_first(&mut self) -> Option<Self::Target> {
        match self.head.clone() {
            None => { None }
            Some(node) => {
                self.size -= 1;
                self.head = node.clone().borrow().next.clone();
                // 头尾相同
                if self.check_node_is_tail(Some(node.clone())) {
                    self.tail = None;
                }
                return Some(node.clone().borrow().data);
            }
        }
    }

    fn del_last(&mut self) -> Option<Self::Target> {
        if self.size == 1 {
            return self.del_first();
        }

        let mut current = self.head.clone();
        let mut prev = None;
        while let Some(node) = current {
            if node.borrow().is_last() {
                current = Some(node);
            } else {
                current = node.borrow().get_next();
                prev = Some(node.clone());
            }
        }

        match prev {
            None => { None }
            Some(node) => {
                self.size -= 1;
                node.clone().borrow_mut().update_next(None);
                let data = self.tail.take().unwrap().borrow().data;
                self.tail = Some(node);
                Some(data)
            }
        }
    }

    fn del_first_value(&mut self, value: Self::Target) -> Option<Self::Target> {
        let mut current = self.head.clone();
        let mut prev = None;

        while let Some(node) = current {
            if *node.borrow().get_ref_data() == value {
                current = Some(node);
                break;
            }

            current = node.borrow().get_next();
            prev = Some(node);
        }

        match current {
            None => { None }
            Some(node) => {
                self.size -= 1;
                // 头节点
                self.update_head(Some(node.clone()));

                // 尾节点
                if self.update_tail(Some(node.clone()),prev.clone()) {
                    return Some(node.borrow().get_data());
                }
                // 中间节点
                prev.unwrap().borrow_mut().update_next(node.borrow().get_next());
                return Some(node.borrow().data);
            }
        }
    }

    fn del_last_value(&mut self, value: Self::Target) -> Option<Self::Target> {
        let mut current = self.head.clone();
        let mut prev = None;

        let mut history_prev=None;
        let mut history_current=None;
        while let Some (node)=current{
            if *node.borrow().get_ref_data() == value {
                history_prev=prev.clone();
                history_current=Some(node.clone())
            }

            current = node.borrow().get_next();
            prev = Some(node);
        }

        match history_current {
            None => { None }
            Some(node) => {
                self.size -= 1;
                // 头节点
                self.update_head(Some(node.clone()));

                // 尾节点
                if self.update_tail(Some(node.clone()),history_prev.clone()) {
                    return Some(node.borrow().get_data());
                }
                // 中间节点
                history_prev.unwrap().borrow_mut().update_next(node.borrow().get_next());
                Some(node.borrow().data)
            }
        }

    }
}