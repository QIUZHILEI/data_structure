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
    fn to_raw(self) -> *mut Self {
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
            let mut node = Node::new(Some(ele)).to_raw();
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
                Box::from_raw(tmp);
                (*tmp).next
            };
            if tmp.is_null(){
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
