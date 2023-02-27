use std::{fmt::Display, marker::PhantomData};

struct Node<T> {
    next: *mut Node<T>,
    prev: *mut Node<T>,
    ele: Option<T>,
}

impl<T> Node<T> {
    fn new_raw(e: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            ele: Some(e),
        }))
    }
}
impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            ele: None,
        }
    }
}

// LinkedList
pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: u64,
    marker: PhantomData<Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        let node = Box::into_raw(Box::new(Node::default()));
        Self {
            head: node,
            tail: node,
            size: 0,
            marker: PhantomData::default(),
        }
    }
    pub fn push_back(&mut self, ele: T) {
        let mut node = Node::new_raw(ele);
        unsafe {
            (*node).prev = self.tail;
            (*self.tail).next = node;
            self.tail = node;
        }
        self.size += 1;
    }
    pub fn push_front(&mut self, ele: T) {
        if self.size == 0 {
            self.push_back(ele);
        } else {
            let mut node = Box::into_raw(Box::new(Node::default()));
            unsafe {
                (*self.head).ele = Some(ele);
                (*node).next = self.head;
                (*self.head).prev = node;
            }
            self.head = node;
            self.size += 1;
        }
    }
    pub fn traverse(&self)
    where
        T: Display,
    {
        let mut tmp = unsafe { (*self.head).next };
        while !tmp.is_null() {
            let ele = unsafe { (*tmp).ele.as_ref() };
            println!("{}", ele.unwrap());
            tmp = unsafe { (*tmp).next };
        }
    }
    pub fn find(&self, other: &T) -> Option<u64>
    where
        T: Eq,
    {
        let mut index = 1;
        let mut tmp = unsafe { (*self.head).next };
        while !tmp.is_null() {
            let ele = unsafe { (*tmp).ele.as_ref() }.unwrap();
            if ele.eq(other) {
                return Some(index);
            }
            tmp = unsafe { (*tmp).next };
            index += 1;
        }
        None
    }
    pub fn first(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            let first = unsafe { (*self.head).next };
            unsafe { (*first).ele.as_ref() }
        }
    }
    pub fn last(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            unsafe { (*self.tail).ele.as_ref() }
        }
    }
    pub fn remove(&mut self, index: u64) -> Option<T> {
        if index > self.size || index == 0 {
            None
        } else {
            if index == 1 {
                self.drop_first()
            } else if index == self.size {
                self.drop_last()
            } else {
                let mut obs = 0;
                let mut pos;
                if index > self.size / 2 {
                    pos = self.tail;
                    while (self.size - obs) != index {
                        pos = unsafe { (*pos).prev };
                        obs += 1;
                    }
                } else {
                    pos = self.head;
                    while obs != index {
                        pos = unsafe { (*pos).next };
                        obs += 1;
                    }
                }
                let next = unsafe { (*pos).next };
                let prev = unsafe { (*pos).prev };
                self.size -= 1;
                unsafe {
                    (*next).prev = prev;
                    (*prev).next = next;
                    let ele = (*pos).ele.take();
                    Box::from_raw(pos);
                    ele
                }
            }
        }
    }
    pub fn insert(&mut self, index: u64, ele: T) {
        assert!(index <= self.size + 1 && index > 0);
        if index == 1 {
            self.push_front(ele);
        } else if index == self.size + 1 {
            self.push_back(ele);
        } else {
            let mut obs = 0;
            let mut pos;
            if index > self.size / 2 {
                pos = self.tail;
                while (self.size - obs) != index {
                    pos = unsafe { (*pos).prev };
                    obs += 1;
                }
            } else {
                pos = self.head;
                while obs != index {
                    pos = unsafe { (*pos).next };
                    obs += 1;
                }
            }
            let mut node = Node::new_raw(ele);
            let mut prev = unsafe { (*pos).prev };
            unsafe {
                (*node).prev = prev;
                (*node).next = pos;
                (*prev).next = node;
                (*pos).prev = node;
            }
            self.size += 1;
        }
    }
    pub fn drop_last(&mut self) -> Option<T> {
        if self.size != 0 {
            let tail = self.tail;
            self.tail = unsafe { (*self.tail).prev };
            self.size -= 1;
            unsafe {
                let ele = (*tail).ele.take();
                Box::from_raw(tail);
                ele
            }
        } else {
            None
        }
    }
    pub fn drop_first(&mut self) -> Option<T> {
        if self.size != 0 {
            if self.size == 1 {
                self.drop_last()
            } else {
                let first = unsafe { (*self.head).next };
                let second = unsafe { (*first).next };
                unsafe {
                    (*self.head).next = second;
                    (*second).prev = self.head;
                }
                self.size -= 1;
                let ele = unsafe { (*first).ele.take() };
                unsafe {
                    Box::from_raw(first);
                }
                ele
            }
        } else {
            None
        }
    }
    pub fn size(&self) -> u64 {
        self.size
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut tmp = self.head;
        loop {
            tmp = unsafe {
                let next = (*tmp).next;
                Box::from_raw(tmp);
                next
            };
            if tmp.is_null() {
                break;
            }
        }
    }
}

pub struct Queue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: u64,
    marker: PhantomData<Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let node = Box::into_raw(Box::new(Node::default()));
        Self {
            head: node,
            tail: node,
            size: 0,
            marker: PhantomData::default(),
        }
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn enqueue(&mut self, e: T) {
        let node = Node::new_raw(e);
        unsafe {
            (*node).prev = self.tail;
            (*self.tail).next = node;
        }
        self.tail = node;
        self.size += 1;
    }
    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let last = self.tail;
        self.tail = unsafe { (*self.tail).prev };
        let res = unsafe { (*last).ele.take() };
        unsafe { Box::from_raw(last) };
        self.size -= 1;
        res
    }
    pub fn first(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            let first = unsafe { (*self.head).next };
            unsafe { (*first).ele.as_ref() }
        }
    }
    pub fn last(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            unsafe { (*self.tail).ele.as_ref() }
        }
    }
    pub fn contains(&self, other: &T) -> bool
    where
        T: Eq,
    {
        let mut tmp = unsafe { (*self.head).next };
        while !tmp.is_null() {
            let self_ele = unsafe { (*tmp).ele.as_ref() }.unwrap();
            if self_ele.eq(other) {
                return true;
            }
            tmp = unsafe { (*tmp).next };
        }
        false
    }
    pub fn traverse(&self)
    where
        T: Display,
    {
        let mut tmp = unsafe { (*self.head).next };
        while !tmp.is_null() {
            let self_ele = unsafe { (*tmp).ele.as_ref() }.unwrap();
            println!("{self_ele}");
            tmp = unsafe { (*tmp).next };
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let mut tmp = self.head;
        loop {
            tmp = unsafe {
                let next = (*tmp).next;
                Box::from_raw(tmp);
                next
            };
            if tmp.is_null() {
                break;
            }
        }
    }
}

pub struct Stack<T> {
    top: *mut Node<T>,
    size: u64,
    marker: PhantomData<Node<T>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        let node = Box::into_raw(Box::new(Node::default()));
        Self {
            top: node,
            size: 0,
            marker: PhantomData::default(),
        }
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let first = unsafe { (*self.top).next };
        if self.size == 1 {
            unsafe {
                (*self.top).next = std::ptr::null_mut();
            }
        } else {
            unsafe {
                (*self.top).next = (*first).next;
            }
        }
        let res = unsafe { (*first).ele.take() };
        unsafe { Box::from_raw(first) };
        res
    }
    pub fn push(&mut self) {}
    pub fn top(&self) -> Option<&T> {
        if self.size==0{
            return None;
        }
        unsafe{(*(*self.top).next).ele.as_ref()}
    }
    pub fn contains(&self, other: &T) -> bool
    where
        T: Eq,
    {
        let mut tmp = unsafe { (*self.top).next };
        while !tmp.is_null() {
            let self_ele = unsafe { (*tmp).ele.as_ref() }.unwrap();
            if self_ele.eq(other) {
                return true;
            }
            tmp = unsafe { (*tmp).next };
        }
        false
    }
    pub fn traverse(&self)
    where
        T: Display,
    {
        let mut tmp = unsafe { (*self.top).next };
        while !tmp.is_null() {
            let self_ele = unsafe { (*tmp).ele.as_ref() }.unwrap();
            println!("{self_ele}");
            tmp = unsafe { (*tmp).next };
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut tmp = self.top;
        loop {
            tmp = unsafe {
                let next = (*tmp).next;
                Box::from_raw(tmp);
                next
            };
            if tmp.is_null() {
                break;
            }
        }
    }
}
