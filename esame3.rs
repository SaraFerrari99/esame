mod esame3;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;



#[derive(Debug)]
pub struct SharedQueue {
    data: Rc<RefCell<Vec<String>>>,
}

impl SharedQueue {
    pub fn new() -> SharedQueue {
        SharedQueue{
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn share(&self)->Self{
        SharedQueue{
            data: self.data.clone(),
        }
    }

    pub fn enqueue(&self,item: String){
        self.data.borrow_mut().push(item);
    }

    pub fn dequeue(&self)->Option<String>{
        if self.data.borrow().is_empty(){
            None
        }else{
            Some(self.data.borrow_mut().remove(0))
        }
    }
}

#[derive(Debug, Eq)]
pub struct Record {
    pub id: String,
    pub score: i32,
}

impl PartialEq for Record{
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Record{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

type TreeLink<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub freq: usize,
    pub left: TreeLink<T>,
    pub right: TreeLink<T>,
}

impl<T: PartialOrd + PartialEq> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem,
            freq: 1,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, elem: T) {
        if elem > self.elem {
            match self.right {
                None => {
                    self.right = Some(Box::new(Node::new(elem)));
                }
                Some(ref mut node) => {
                    node.insert(elem);
                }
            }
        }else if elem == self.elem {
            self.freq += 1
        }else if elem < self.elem {
            match self.left {
                None => {
                    self.left = Some(Box::new(Node::new(elem)));
                }
                Some(ref mut node) => {
                    node.insert(elem);
                }
            }
        }
    }

    pub fn frequency_of(&self, elem: &T) -> usize {
        if self.elem == *elem {
            return self.freq
        }else if *elem > self.elem {
            match self.left {
                None => {
                    0
                }
                Some(ref node) => {
                    node.frequency_of(elem)
                }
            }
        }else{
            match self.right {
                None => {
                    0
                }
                Some(ref node) => {
                    node.frequency_of(elem)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct FrequencyTree<T> {
    pub root: TreeLink<T>,
}

impl<T: PartialOrd + PartialEq> FrequencyTree<T> {
    pub fn new() -> Self {
        FrequencyTree { root: None }
    }

    pub fn insert(&mut self, elem: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(Node::new(elem)));
            }
            Some(ref mut node) => {
                node.insert(elem);
            }
        }
    }

    pub fn frequency_of(&self,elem: &T)->usize{
        match self.root {
            None => {
                0
            }
            Some(ref node) => {
                node.frequency_of(elem)
            }
        }
    }
}

// TEST ESERCIZIO 1
fn main() {
    let q1 = SharedQueue::new();
    let q2 = q1.share();

    q1.enqueue("Messaggio 1".to_string());
    q2.enqueue("Messaggio 2".to_string());

    // q1 e q2 condividono gli stessi dati
    println!("{:?}", q1.dequeue());
    println!("{:?}", q2.dequeue());
    println!("{:?}", q1.dequeue());
}