use std::{fmt::Display, marker::PhantomData, ptr::NonNull};

struct Node<T> {
    next: *mut Node<T>,
    prev: *mut Node<T>,
    ele: Option<T>,
}

impl<T> Node<T> {
    fn new(e: Option<T>) -> Self {
        match e {
            Some(_) => Self {
                next: std::ptr::null_mut(),
                prev: std::ptr::null_mut(),
                ele: e,
            },
            None => Self {
                next: std::ptr::null_mut(),
                prev: std::ptr::null_mut(),
                ele: None,
            },
        }
    }
    fn to_raw(mut self) -> *mut Self {
        Box::into_raw(Box::new(self))
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
        let node = Node::new(None).to_raw();
        Self {
            head: node,
            tail: node,
            size: 0,
            marker: PhantomData::default(),
        }
    }
    pub fn push_back(&mut self, ele: T) {
        let mut node = Node::new(Some(ele)).to_raw();
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
            let mut node = Node::new(None).to_raw();
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
    pub fn remove(&mut self, index: u64) {

    }
    pub fn insert(&mut self, index: u64) {}
    pub fn drop_last(&mut self) {

    }
    pub fn drop_first(&mut self) {
        
    }
    pub fn size(&self) {}
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {}
}

pub struct Queue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: u64,
    marker: PhantomData<Node<T>>,
}

pub struct Dequeue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: u64,
    marker: PhantomData<Node<T>>,
}

pub struct Stack<T> {
    top: *mut Node<T>,
    size: u64,
    marker: PhantomData<Node<T>>,
}
