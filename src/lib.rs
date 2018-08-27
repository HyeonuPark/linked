
use std::ptr;

#[derive(Debug)]
pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

#[derive(Debug)]
struct Node<T> {
    xor_ptr: usize,
    value: T,
}

#[derive(Debug)]
pub struct Cursor<'a, T: 'a> {
    prev: *mut Node<T>,
    next: *mut Node<T>,
    list: &'a mut List<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn cursor_head(&mut self) -> Cursor<T> {
        Cursor {
            prev: ptr::null_mut(),
            next: self.head,
            list: self,
        }
    }

    pub fn cursor_tail(&mut self) -> Cursor<T> {
        Cursor {
            prev: self.tail,
            next: ptr::null_mut(),
            list: self,
        }
    }
}

impl<'a, T> Cursor<'a, T> {
    fn next_mut<'b>(&mut self) -> Option<&'b mut Node<T>> {
        unsafe {
            self.next.as_mut()
        }
    }

    fn prev_mut<'b>(&mut self) -> Option<&'b mut Node<T>> {
        unsafe {
            self.prev.as_mut()
        }
    }

    pub fn next(&mut self) -> Option<&mut T> {
        self.next_mut().map(|node| {
            let next = (node.xor_ptr ^ self.prev as usize) as *mut Node<T>;

            self.prev = self.next;
            self.next = next;

            &mut node.value
        })
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        self.prev_mut().map(|node| {
            let prev = (node.xor_ptr ^ self.next as usize) as *mut Node<T>;

            self.next = self.prev;
            self.prev = prev;

            &mut node.value
        })
    }

    fn insert(&mut self, value: T) -> *mut Node<T> {
        let node = Box::leak(Box::new(Node {
            xor_ptr: self.prev as usize ^ self.next as usize,
            value,
        })) as *mut Node<T>;

        if let Some(prev) = self.prev_mut() {
            prev.xor_ptr ^= self.next as usize;
            prev.xor_ptr ^= node as usize;
        } else {
            self.list.head = node;
        }

        if let Some(next) = self.next_mut() {
            next.xor_ptr ^= self.prev as usize;
            next.xor_ptr ^= node as usize;
        } else {
            self.list.tail = node;
        }

        node
    }

    pub fn insert_next(&mut self, value: T) {
        self.next = self.insert(value)
    }

    pub fn insert_prev(&mut self, value: T) {
        self.prev = self.insert(value)
    }

    fn remove(&mut self, node: *mut Node<T>) -> T {
        if let Some(next) = self.next_mut() {
            next.xor_ptr ^= node as usize;
            next.xor_ptr ^= self.prev as usize;
        } else {
            self.list.tail = self.prev;
        }

        if let Some(prev) = self.prev_mut() {
            prev.xor_ptr ^= node as usize;
            prev.xor_ptr ^= self.next as usize;
        } else {
            self.list.head = self.next;
        }

        unsafe {
            Box::from_raw(node).value
        }
    }

    pub fn remove_next(&mut self) -> Option<T> {
        self.next_mut().map(|next| {
            self.next = (next.xor_ptr ^ self.prev as usize) as *mut Node<T>;
            self.remove(next)
        })
    }

    pub fn remove_prev(&mut self) -> Option<T> {
        self.prev_mut().map(|prev| {
            self.prev = (prev.xor_ptr ^ self.next as usize) as *mut Node<T>;
            self.remove(prev)
        })
    }

    pub fn split_before(&mut self) -> List<T> {
        let before = List {
            head: self.list.head,
            tail: self.prev,
        };

        self.list.head = self.next;
        self.prev = ptr::null_mut();

        before
    }

    pub fn split_after(&mut self) -> List<T> {
        let after = List {
            head: self.next,
            tail: self.list.tail,
        };

        self.list.tail = self.prev;
        self.next = ptr::null_mut();

        after
    }
}
