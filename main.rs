use std::sync::{Arc, Mutex};
use std::fmt::{Debug, Display};

// ============================================================================
// SCHELETRO ESERCIZIO 1: SERVER METRICS (Arc + Mutex)
// ============================================================================

pub struct ServerMetrics {
    data: Mutex<(usize, Option<u16>)>,
}

impl ServerMetrics {
    pub fn new() -> Self {
        ServerMetrics{
            data: Mutex::new((0, None)),
        }
    }

    pub fn increment_requests(&self) {
        // 1. Acquisiamo il lock sul Mutex. Finché "guard" è vivo, nessun altro thread può toccare i dati.
        let mut data = self.data.lock().unwrap();
        data.0 += 1;
    }

    pub fn register_error(&mut self, error_code: u16) {
        let mut data = self.data.lock().unwrap();
        if error_code > 0{
            data.1 = None;
        }else{
            data.1 = Some(error_code);
        }
    }

    pub fn get_snapshot(&self) -> (usize, Option<u16>) {
       let data = self.data.lock().unwrap();
        *data
    }
}


// ============================================================================
// SCHELETRO ESERCIZIO 2: EXPRESSION BINARY TREE
// ============================================================================

type NodeLink<T> = Option<Box<ExpressionNode<T>>>;

#[derive(Debug)]
pub struct ExpressionNode<T> {
    pub token: T, // Può essere una stringa come "+", "-", o un numero "5"
    pub left: NodeLink<T>,
    pub right: NodeLink<T>,
}

impl <T: PartialEq> PartialEq for ExpressionNode<T> {
    fn eq(&self, other: &ExpressionNode<T>) -> bool {
        self.token == other.token
    }
}

impl<T: std::cmp::PartialEq> ExpressionNode<T> {
    pub fn new_leaf(token: T) -> Self {
        ExpressionNode {
            token,
            left: None,
            right: None,
        }
    }

    pub fn new_operator(token: T, left: ExpressionNode<T>, right: ExpressionNode<T>) -> Self {
        ExpressionNode {
            token,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    pub fn count_leaves(&self) -> usize {

        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        let mut count = 0;

        if let Some(left) = &self.left {
            count += left.count_leaves();
        }else if let Some(right) = &self.right {
            count += right.count_leaves();
        }

        count
    }

    pub fn contains_operator(&self, op: &T) -> bool{

        if self.token == *op {
            return true;
        }

        if let Some(ref left) = self.left {
            if left.contains_operator(op) {
                return true;
            }
        }
        if let Some(ref right) = self.right {
            if right.contains_operator(op) {
                return true;
            }

        }
        false
    }
}

#[derive(Debug)]
pub struct ExpressionTree<T> {
    pub root: NodeLink<T>,
}

impl<T: PartialEq + Display + Debug> ExpressionTree<T> {
    pub fn new(root_node: ExpressionNode<T>) -> Self {
        ExpressionTree{
            root: Some(Box::new(root_node)),
        }
    }

    pub fn count_leaves(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref node) =>{
                node.count_leaves()
            }
        }
    }

    pub fn contains_operator(&self, op: &T) -> bool {

        match self.root {
            None => false,
            Some(ref node) => {
                if node.token == *op {
                    true
                }else{
                    node.contains_operator(op)
                }
            }
        }
    }
}


// ============================================================================
// TEST SUITE DI VERIFICA (MAIN)
// ============================================================================
fn main() {
    println!("--- TEST ESERCIZIO 1 (Multi-threading) ---");
    let metrics = Arc::new(ServerMetrics::new());
    let mut threads = vec![];

    // Spawnamo 3 thread concorrenti
    for _ in 0..3 {
        let mut m_clone = Arc::clone(&metrics);
        threads.push(std::thread::spawn(move || {
            m_clone.increment_requests();
            m_clone.register_error(500);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let (reqs, err) = metrics.get_snapshot();
    println!("Richieste totali (atteso: 3): {}", reqs);
    println!("Ultimo errore (atteso: Some(500)): {:?}", err);


    println!("\n--- TEST ESERCIZIO 2 (Albero d'Espressione) ---");
    // Costruiamo l'espressione: (5 + 3)
    let n5 = ExpressionNode::new_leaf("5".to_string());
    let n3 = ExpressionNode::new_leaf("3".to_string());
    let radice = ExpressionNode::new_operator("+".to_string(), n5, n3);

    let albero = ExpressionTree::new(radice);

    println!("Numero di foglie (atteso: 2): {}", albero.count_leaves());
    println!("Contiene l'operatore '+'? (atteso: true): {}", albero.contains_operator(&"+".to_string()));
    println!("Contiene l'operatore '-'? (atteso: false): {}", albero.contains_operator(&"-".to_string()));
}