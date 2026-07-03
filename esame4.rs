// ============================================================================
// SCHELETRO ESERCIZIO 1: DYNAMIC FILTER
// ============================================================================

use std::ops::Deref;

// Tipo di comodo per rappresentare una closure di filtro allocata nell'heap
type FilterClosure<'a> = Box<dyn Fn(&str) -> bool + 'a>;

pub struct DynamicFilter<'a> {
    pub filters: Vec<FilterClosure<'a>>,
}

impl<'a> DynamicFilter<'a> {
    pub fn new() -> Self {
        DynamicFilter{
            filters: Vec::new(),
        }
    }

    pub fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&str) -> bool + 'a
    {
        self.filters.push(Box::new(filter));
    }

    // 3. apply consuma il vettore di stringhe in input, applica la catena di filtri e raccoglie il risultato
    pub fn apply(&self, items: Vec<String>) -> Vec<String> {
        items.into_iter()
            .filter(|item| {
                // Scorriamo tutti i filtri presenti nel vettore.
                // .all() restituisce true solo se OGNI filtro f(item) restituisce true.
                self.filters.iter().all(|f| f(item))
            })
            .collect() // Rimpacchetta le stringhe superstiti in un Vec<String>
    }
}


// ============================================================================
// SCHELETRO ESERCIZIO 2: CACHE LINKED LIST
// ============================================================================

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

// TODO: Implementa il trait PartialEq per Node<T> in base a quanto richiesto
impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.elem == other.elem
    }
}

impl<T: std::cmp::PartialEq> Node<T> {
    pub fn new(elem: T) -> Self {
        Node{elem: elem, next: None}
    }

    fn remove_last(&mut self, elem: T){
        // Guardiamo il nodo successivo //node e il nodo successivo
        if let Some(ref mut node) = self.next {
            // Se il nodo DOPO il prossimo è None, significa che next_node è l'ULTIMO.
            // Quindi noi siamo il PENULTIMO nodo!
            if node.next.is_none() {
                self.next = None;
            }else{
                node.remove_last(elem);
            }
        }
    }

    fn contains(&self, elem: &T) -> bool {
        if self.elem == *elem {
            true
        }else{
            if let Some(ref node) = self.next{//RICORDARSI QUESTO O CHIAMI ALL INFINITO LO STESSO NODO
                node.contains(elem)
            }else{
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct CacheList<T> {
    pub head: Link<T>,
    pub capacity: usize,
    pub len: usize,
}

impl<T: PartialEq + Clone> CacheList<T> {
    pub fn new(capacity: usize) -> Self {
        CacheList{head: None, capacity, len: 0}
    }

    pub fn push_front(&mut self, elem: T) {
        self.head = Some(Box::new(Node{elem: elem.clone(), next: self.head.take()}));
        if self.len >= self.capacity {
            if let Some(ref mut node) = self.head {
                node.remove_last(elem);
            }
        }else{
            self.len += 1;
        }
    }

    pub fn contains(&self, elem: &T) -> bool {
        match self.head {
            None => false,
            Some(ref node) => node.contains(elem),
        }
    }
}

// ============================================================================
// TEST SUITE DI VERIFICA (MAIN)
// ============================================================================
fn main() {
    println!("--- TEST ESERCIZIO 1 ---");
    let mut df = DynamicFilter::new();
    df.add_filter(|s| s.len() > 3);
    df.add_filter(|s| s.contains('a'));

    let parole = vec![
        "casa".to_string(), // OK (>3 caratteri, contiene 'a')
        "re".to_string(),   // NO (troppo corta)
        "muro".to_string(), // NO (non contiene 'a')
        "gatto".to_string(),// OK (>3 caratteri, contiene 'a')
    ];

    let filtrate = df.apply(parole);
    println!("Risultato (atteso: [\"casa\", \"gatto\"]): {:?}", filtrate);


    println!("\n--- TEST ESERCIZIO 2 ---");
    let mut cache = CacheList::new(2); // Capacità massima = 2 elementi

    cache.push_front("Elemento 1");
    cache.push_front("Elemento 2");
    cache.push_front("Elemento 3"); // Questo inserimento deve far eliminare "Elemento 1"

    println!("Contiene 'Elemento 3'? (atteso: true): {}", cache.contains(&"Elemento 3"));
    println!("Contiene 'Elemento 2'? (atteso: true): {}", cache.contains(&"Elemento 2"));
    println!("Contiene 'Elemento 1'? (atteso: false): {}", cache.contains(&"Elemento 1"));
}