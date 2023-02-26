//!TODO(Seq List)
use std::{ptr::NonNull, marker::PhantomData};

pub struct List<T>{
    data:NonNull<T>,
    len:u64,
    cap:u64,
    marker:PhantomData<Box<T>>
}

pub struct Queue<T>{
    data:NonNull<T>,
    head:u64,
    tail:u64,
    len:u64,
    cap:u64,
    marker:PhantomData<Box<T>>
}

pub struct Dequeue<T>{
    data:NonNull<T>,
    head:u64,
    tail:u64,
    len:u64,
    cap:u64,
    marker:PhantomData<Box<T>>
}

pub struct Stack<T>{
    data:NonNull<T>,
    len:u64,
    cap:u64,
    top:u64,
    marker:PhantomData<Box<T>>
}


