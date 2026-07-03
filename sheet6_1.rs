use std::fmt::Debug;
use crate::Nodo;

#[derive(Debug)]
struct TreeNode<T:PartialOrd + Clone + Debug> {
    valore: T,
    sinistro: Option<Box<TreeNode<T>>>,
    destro: Option<Box<TreeNode<T>>>,
}

impl<T:PartialOrd + Clone + Debug> TreeNode<T> {
    pub fn new(value: T)->Self{
        TreeNode{
            valore: value,
            sinistro: None,
            destro: None
        }
    }

    pub fn insert(&mut self, value: T){
        if value < self.valore{
            match &mut self.sinistro{
                Some(sinistro) => sinistro.insert(value),
                None => {self.sinistro = Some(Box::new(TreeNode::new(value)));}
            }
        }else{
            match &mut self.destro {
                Some(destro) => destro.insert(value),
                None => {self.destro = Some(Box::new(TreeNode::new(value)));}
            }
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        // Trasformiamo il vec in un iteratore che "consuma" gli elementi,
        // restituendo direttamente T e non &T.
        let mut iter = vec.into_iter();

        // Estraiamo il primo elemento per fare la radice.
        // Usiamo expect per gestire in modo sicuro un vettore vuoto.
        let primo_valore = iter.next().expect("Errore: il vettore non puo' essere vuoto");
        let mut tree = Self::new(primo_valore);

        // Cicliamo sui rimanenti elementi (il primo è già stato rimosso dall'iteratore)
        for item in iter {
            tree.insert(item);
        }

        // Restituiamo l'albero (in Rust l'espressione finale senza punto e virgola funge da return)
        tree
    }
}

