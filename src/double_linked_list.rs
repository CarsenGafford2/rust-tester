struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

pub struct DoubleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    size: u32,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: None,
            prev: None,
        });
        match self.head.as_mut() {
            Some(head) => {
                let mut current = head;
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
            None => self.head = Some(new_node),
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let result = match self.head.as_mut() {
            None => None,
            Some(head) if head.next.is_none() => self.head.take(),
            Some(head) => {
                let mut current = head;
                while current.next.as_ref().and_then(|next| next.next.as_ref()).is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next.take()
            }
        };

        result.map(|node| {
            self.size -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        let mut current = self.head.as_ref()?;
        while let Some(next) = current.next.as_ref() {
            current = next;
        }
        Some(&current.value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = self.head.as_ref();
        for _ in 0..index {
            current = current?.next.as_ref();
        }
        current.map(|node| &node.value)
    }

    pub fn length(&self) -> u32 {
        self.size
    }

    pub fn add(&mut self, index: usize, value: T) {
        if index > self.size as usize {
            return;
        }
        let mut new_node = Box::new(Node {
            value,
            next: None,
            prev: None,
        });
        if index == 0 {
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();
            for _ in 0..index - 1 {
                current = current.next.as_mut().unwrap();
            }
            new_node.next = current.next.take();
            current.next = Some(new_node);
        }
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size as usize {
            return None;
        }
        let removed_node = if index == 0 {
            self.head.take().unwrap()
        } else {
            let mut current = self.head.as_mut().unwrap();
            for _ in 0..index - 1 {
                current = current.next.as_mut().unwrap();
            }
            current.next.take().unwrap()
        };
        self.size -= 1;
        Some(removed_node.value)
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    pub fn reverse(&mut self) {
        let mut current = self.head.take();
        let mut prev: Option<Box<Node<T>>> = None;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            node.prev = None;
            prev = Some(node);
            current = next;
        }

        self.head = prev;
        self.tail = None;
    }
}