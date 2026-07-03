use std::cell::RefCell;
use std::rc::Rc;

struct EntangledBit{
        bit : Rc<RefCell<bool>>,
}

impl EntangledBit {
    fn new(bit: Rc<RefCell<bool>>) -> EntangledBit{
        EntangledBit{
            bit : bit,
        }
    }

    fn default()->EntangledBit{
        EntangledBit{
            bit : Rc::new(RefCell::new(false)),
        }
    }

    fn get(&self) -> bool{
        *self.bit.borrow()
    }
    fn set(&mut self){
        *self.bit.borrow_mut() = true;
    }

    fn reset(self){
        *self.bit.borrow_mut() = false;
    }

    fn entagle_with(&self, other: &mut Self){
        other.bit = self.bit.clone();
    }
}