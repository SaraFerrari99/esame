use std::cmp::Ordering;
use std::ops::DerefMut;

struct Libro{
    isbn: u32,
    titolo: String,
}

struct NodoLibro {
    valore: Libro,
    sinistra: Option<Box<NodoLibro>>,
    destra: Option<Box<NodoLibro>>,
}

struct Catalogo{
    radice: Option<Box<NodoLibro>>,
}

impl PartialEq for Libro{
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

impl PartialOrd for Libro{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.isbn.partial_cmp(&other.isbn)
    }
}

impl Libro{
    fn new(isbn: u32) -> Libro{
        Libro{
            isbn,
            titolo: "".to_string(),
        }
    }
}

impl NodoLibro {
    fn new(libro: Libro) -> NodoLibro {
        NodoLibro{
            valore: libro,
            sinistra: None,
            destra: None,
        }
    }

    fn inserisci(&mut self,libro: Libro){
        if libro.isbn < self.valore.isbn {
            match &mut self.sinistra {
                None => {
                    self.sinistra = Some(Box::new(NodoLibro::new(libro)));
                }
                Some(n) => {
                    n.inserisci(libro);
                }
            }
        }else if libro.isbn > self.valore.isbn {
            match &mut self.destra {
                None => {
                    self.destra = Some(Box::new(NodoLibro::new(libro)));
                }
                Some(n) => {
                    n.inserisci(libro);
                }
            }
        }
    }

    fn conta_titoli_lunghi(&self,len: usize)->i32{
        let mut count = 0;
        if &self.valore.titolo.len() > &len {//essendo stato ordinato per isbn poi va controllato tutto l albero perche non ordinato per string
            count += 1;
        }
        if let Some(ref left_node) = self.sinistra {
            count += left_node.conta_titoli_lunghi(len);
        }
        if let Some(ref right_node) = self.destra {
            count += right_node.conta_titoli_lunghi(len);
        }

        count
    }
}

impl Catalogo{
    fn new() -> Self {
        Catalogo{radice: None}
    }

    fn inserisci(&mut self, nuovo_libro: Libro){
        match &mut self.radice {
            None => {
                self.radice = Some(Box::new(NodoLibro::new(nuovo_libro)));
            }
            Some(radice) => {
                radice.inserisci(nuovo_libro);
            }
        }
    }

    fn conta_titoli_lunghi(&self, min_len: usize)->i32{
        match &self.radice {
            Some(radice) => {
                radice.conta_titoli_lunghi(min_len)
            }
            None => 0
        }
    }
}

struct Dispositivo{
    id: u32,
    acceso: bool,
    consumo: u32
}

struct Casa{
    dispositivi: Vec<Dispositivo>,
}


impl Casa{
    fn new()->Casa{
        Casa{dispositivi: Vec::new()}
    }

    fn aggiungi_dispositivo(&mut self, d: Dispositivo){
        self.dispositivi.push(d);
    }

    fn acceso(&mut self) {
        self.dispositivi.iter_mut().for_each(|d|{
            d.acceso = true;
        })
    }

    fn consumo_attivo(&mut self)->u32{
        let mut count = 0;
        self.dispositivi.iter().for_each(|d|{
            if d.acceso {
                count += d.consumo;
            }
        });
        count
    }

    fn rimuovi_energivori(&mut self, soglia: u32) -> Vec<Dispositivo> {
        let mut rimossi = Vec::new();
        let mut i = 0;

        // Scorriamo finché l'indice è minore della lunghezza attuale del vettore
        while i < self.dispositivi.len() {
            if self.dispositivi[i].consumo > soglia {
                // Rimuoviamo il dispositivo dalla Casa.
                // remove(i) estrae l'elemento e lo restituisce, così possiamo fare subito il push!
                let d = self.dispositivi.remove(i);
                rimossi.push(d);

                // ATTENZIONE: non incrementiamo 'i' qui!
                // Rimuovendo l'elemento, quello successivo è scivolato indietro nella posizione 'i'.
            } else {
                // Se non abbiamo rimosso nulla, passiamo tranquillamente al prossimo elemento
                i += 1;
            }
        }

        rimossi
    }
}

impl Dispositivo{
    fn new(id: u32, consumo:u32)->Dispositivo{
        Dispositivo{
            id,
            consumo,
            acceso: false
        }
    }
}


fn main(){

}