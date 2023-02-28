//!TODO(Seq List)
use std::{alloc::Layout, fmt::Display, marker::PhantomData};
const INIT_SIZE: usize = 10;
pub struct List<T> {
    data: *mut T,
    len: usize,
    cap: usize,
    marker: PhantomData<Box<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            data: List::init_data(INIT_SIZE),
            len: 0,
            cap: INIT_SIZE,
            marker: PhantomData::default(),
        }
    }
    pub fn with_cap(init_size: usize) -> Self {
        Self {
            data: List::init_data(init_size),
            len: 0,
            cap: init_size,
            marker: PhantomData::default(),
        }
    }
    fn init_data(size: usize) -> *mut T {
        let layout = match Layout::array::<T>(size) {
            Ok(res) => res,
            Err(err) => {
                println!("{}", err.to_string());
                panic!("layout error!");
            }
        };
        unsafe { std::alloc::alloc(layout) as *mut T }
    }

    fn extend_cap(&mut self) {
        let new_cap = self.cap * 2;
        if new_cap < self.cap {
            panic!("array out of usize bound!");
        }
        let new_layout = match Layout::array::<T>(new_cap) {
            Ok(res) => {
                self.cap = new_cap;
                res
            }
            Err(err) => {
                println!("{}", err.to_string());
                panic!("layout error!");
            }
        };
        let old_layout = Layout::array::<T>(self.cap).unwrap();
        let new_ptr = unsafe {
            std::alloc::realloc(self.data as *mut u8, old_layout, new_layout.size()) as *mut T
        };
        self.data = new_ptr;
    }
    pub fn traverse(&self)
    where
        T: Display,
    {
    }
    pub fn find(&self, ohter: &T) -> usize
    where
        T: Eq,
    {
        0
    }
    pub fn insert(&mut self, ele: T, index: usize) {}
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.len == 0 || index >= self.len {
            return None;
        }
        let res=unsafe{
            self.data.add(index).read()
        };
        let mut pos=index+1;
        while pos<self.len{
            let tmp=unsafe{self.data.add(pos).read()};
            unsafe{self.data.add(pos-1).write(tmp)};
            pos+=1;
        }
        self.len-=1;
        Some(res)
    }
    pub fn pop(&mut self)->Option<T>{
        self.remove(self.len-1)
    }
    pub fn push_back(&mut self) {
        
    }

    pub fn get(&self, index: usize) -> Option<&mut T> {
        if self.len == 0 || index >= self.len {
            return None;
        }
        let res = unsafe { &mut std::ptr::read(self.data.add(index)) };
        Some(res)
    }
    pub fn length(&self) -> usize {
        self.len
    }
}

pub struct Queue<T> {
    data: *mut T,
    first: usize,
    last: usize,
    len: usize,
    cap: usize,
    marker: PhantomData<Box<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {}
    pub fn with_cap() -> Self {}
    pub fn traverse()
    where
        T: Display,
    {
    }
    pub fn contains() -> bool
    where
        T: Eq,
    {
    }
}
pub struct Stack<T> {
    data: *mut T,
    len: usize,
    cap: usize,
    top: usize,
    marker: PhantomData<Box<T>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {}
    pub fn with_cap() -> Self {}
    pub fn traverse()
    where
        T: Display,
    {
    }
    pub fn contains() -> bool
    where
        T: Eq,
    {
    }
}
