#!TODO(instuive list)
struct Link{
    next:NonNull<Link>,
    marker:PhantomData<Box<Link>>
}

struct Node<T>{
    data:Option<T>,
    link:Link
}

pub struct ListHead<T>{
    head:Node,
    size:u64
}

impl<T> ListHead<T>{
    
}