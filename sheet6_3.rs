trait Sound{
    fn make_sound(&self)-> String;

}

struct Dog;
struct Cat;
struct Frog;
struct Cow;

impl Sound for Dog {
    fn make_sound(&self)-> String {
        "Dog".to_string()
    }
}

impl Sound for Cat {
    fn make_sound(&self)-> String {
        "Cat".to_string()
    }
}

impl Sound for Frog {
    fn make_sound(&self)-> String {
        "Frog".to_string()
    }
}
impl Sound for Cow {
    fn make_sound(&self)-> String {
        "Cow".to_string()
    }
}

struct FarmCell{
    element: Box<dyn Sound>,
    next: Option<Box<FarmCell>>,
}

impl FarmCell{
    fn new(element: Box<dyn Sound>) -> Self {
        FarmCell{
            element,
            next: None,
        }
    }

    fn insert(&mut self, element: Box<dyn Sound>) {
        match self.next {
            Some(ref mut next) => next.insert(element),
            None => self.next = Some(Box::new(FarmCell::new(element))),
        }
    }
}

impl Sound for FarmCell {
    fn make_sound(&self)-> String {
        let mut result = self.element.make_sound();
        if let Some(ref next) = self.next {
            result.push_str(&next.make_sound());
        }
        result
    }
}