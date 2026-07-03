/*use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct SharedCommunications{
    message : Rc<RefCell<Option<String>>>,
}

impl SharedCommunications{
    fn new() -> SharedCommunications{
        SharedCommunications{
            message : Rc::new(RefCell::new(None)),
        }
    }

    fn new_form(other: &Self) -> Self{
        Self{
            message : other.message.clone(),
        }
    }

    fn send(&mut self, message:String) -> Result<(), ()>{
        if self.message.borrow().is_some() {
            return Err(());
        }else{
            self.message.replace(Some(message));
            return Ok(());
        }
    }

    fn receive(&mut self) -> Option<String>{
        if self.message.borrow().is_some() {
            return Some(self.message.borrow().clone().unwrap());
        }else{
            return None;
        }
    }
}*/

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Content {
    pub i: i32,
    pub s: String,
}

impl Content {
    pub fn new(i: i32, s: String) -> Content {
        Content { i, s }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Content) -> bool {
        self.s.len() == other.s.len()
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.s.len().partial_cmp(&other.s.len())
    }
}

// Suggerimento: dovrai implementare PartialEq e PartialOrd per Content qui

type TreeLink<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node {
            elem,
            left: None,
            center: None,
            right: None,
        }
    }
}

impl <T: PartialOrd> Node<T>{
    pub fn insert(&mut self, elem: T) {
        if elem < self.elem {
            match &mut self.left{
                None => {
                    self.left = Some(Box::new(Node::new(elem)));
                }
                Some(node) => {
                    node.insert(elem);
                }
            }
        }else{
            if elem > self.elem {
                match &mut self.right{
                    None => {
                        self.right = Some(Box::new(Node::new(elem)));
                    }
                    Some(node) => {
                        node.insert(elem);
                    }
                }
            }else{
                if elem == self.elem {
                    match &mut self.center{
                        None => {
                            self.center = Some(Box::new(Node::new(elem)));
                        }
                        Some(node) => {
                            node.insert(elem);
                        }
                    }
                }
            }
        }
    }

    pub fn howMany_g(&self, el: &T)-> i32{
        let mut count = 0;
        if &self.elem > el {
            count += 1;

            // Dobbiamo esplorare tutti i rami perché potrebbero contenere altri nodi maggiori del target
            if let Some(ref left_node) = self.left {
                count += left_node.howMany_g(el);
            }
            if let Some(ref center_node) = self.center {
                count += center_node.howMany_g(el);
            }
            if let Some(ref right_node) = self.right {
                count += right_node.howMany_g(el);
            }
        } else {
            if let Some(ref right_node) = self.right {
                count += right_node.howMany_g(el);
            }
        }

        count
    }
}

#[derive(Debug)]
pub struct Tree<T> {
    root: TreeLink<T>,
}

// Suggerimento: dovrai implementare i metodi per Tree<T> qui
impl <T: PartialOrd> Tree<T> {
    fn new() -> Tree<T> {
        Tree { root: None }
    }

    fn add(&mut self, el: T) {
        match &mut self.root{
            None => {
                self.root = Some(Box::new(Node::new(el)));
            }
            Some(node) => {
                node.insert(el);
            }
        }
    }

    fn howmany_greater(&self, el: &T)-> i32{
        match &self.root{
            None => 0,
            Some(node) => {
                node.howMany_g(el)
            }
        }
    }


}


/*fn main() {

    let mut c1 = SharedCommunications::new();
    let mut c2 = SharedCommunications::new_form(&c1);
    let mut c3 = SharedCommunications::new_form(&c1);
    let mut c4 = SharedCommunications::new_form(&c3);

    println!("{:?}", c1.receive());                  // Risultato: None
    println!("{:?}", c2.send("hello".to_owned()));   // Risultato: Ok(())
    println!("{:?}", c3.send("hello2".to_owned()));  // Risultato: Err(())
    println!("{:?}", c4.receive());                  // Risultato: Some("hello")
}*/

fn main() {
    // ==========================================
    // TEST 1: Creazione albero e ordinamento di Content
    // ==========================================
    println!("--- TEST 1: Inizializzazione e Confronto ---");
    let mut tree = Tree::new();

    let c_short = Content::new(10, "abc".to_string());   // lung: 3
    let c_medium = Content::new(20, "abcde".to_string()); // lung: 5
    let c_long = Content::new(30, "abcdefg".to_string()); // lung: 7

    println!("È 'abc' < 'abcde'? {}", c_short < c_medium); // Deve stampare: true
    println!("È 'abcdefg' > 'abcde'? {}", c_long > c_medium); // Deve stampare: true
    println!();

    // ==========================================
    // TEST 2: Inserimento elementi (add)
    // ==========================================
    println!("--- TEST 2: Inserimento nell'Albero ---");
    // Inseriamo c_medium come RADICE (lunghezza 5)
    tree.add(c_medium);

    // Inseriamo c_short (lunghezza 3). Andrà a SINISTRA (left)
    tree.add(c_short);

    // Inseriamo c_long (lunghezza 7). Andrà a DESTRA (right)
    tree.add(c_long);

    println!("Albero dopo i primi tre inserimenti:");
    println!("{:#?}", tree);
    println!();

    // ==========================================
    // TEST 3: Gestione dei duplicati (center)
    // ==========================================
    println!("--- TEST 3: Gestione dei Duplicati ---");
    // Creiamo un altro elemento con lunghezza 5 (uguale alla radice)
    let c_equal = Content::new(99, "abcde".to_string()); // lung: 5

    // Deve finire nel ramo CENTRALE (center) della radice
    tree.add(c_equal);

    println!("Albero dopo l'inserimento del duplicato nel centro:");
    println!("{:#?}", tree);
    println!();

    // ==========================================
    // TEST 4: Conteggio elementi maggiori (howmany_greater)
    // ==========================================
    println!("--- TEST 4: howmany_greater ---");
    // Inseriamo un altro elemento molto lungo (lunghezza 10) per avere più elementi grandi
    tree.add(Content::new(40, "abcdefghij".to_string())); // lung: 10

    // Creiamo un elemento di confronto con lunghezza 4
    let target = Content::new(0, "aaaa".to_string()); // lung: 4

    // I nodi con lunghezza > 4 nel nostro albero sono:
    // - la radice ("abcde", lung 5)
    // - il duplicato ("abcde", lung 5)
    // - il nodo destro ("abcdefg", lung 7)
    // - il nodo da 10 caratteri ("abcdefghij", lung 10)
    // Risultato atteso: 4
    let maggiori = tree.howmany_greater(&target);
    println!("Quanti nodi hanno una stringa più lunga di 'aaaa'? {}", maggiori);
    println!("Risultato atteso: 4");
}
