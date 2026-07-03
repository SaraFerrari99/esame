use std::rc::Rc;
use std::cell::RefCell;

type CarRef = Rc<RefCell<Car>>;
#[derive(Debug)]
struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool,
}
impl Car {
    pub fn new(model: String, year: u32, price: u32, rent: bool) -> Self {
        Self {
            model,
            year,
            price,
            rent,
        }
    }
    pub fn default() -> Self {
        Self {
            model: "".to_string(),
            year: 0,
            price: 0,
            rent: false,
        }
    }
}

struct CarDealer {
    cars: Vec<CarRef>,
}
struct User {
    car: Option<CarRef>,
}
impl CarDealer {
    pub fn new(cars: Vec<CarRef>) -> Self {
        Self { cars }
    }
    pub fn add_car(&mut self, car: Car) {
        self.cars.push(Rc::new(RefCell::new(car)))
    }
    pub fn print_cars(&mut self) {
        self.cars.iter_mut().for_each(|x| {
            println!("{:?}", x);
        })
    }

    pub fn rent_user(&mut self, user: &mut User, model: String) {
        let mut index = 0;
        let mut found = false;
        for (i, car) in self.cars.iter().enumerate() {
            if car.borrow_mut().model == model {
                index = i;
                if !found {
                    found = true;
                }
            }
        }
        if found {
            let mut clone_car: CarRef = Rc::clone(&self.cars[index].clone());
            println!("index: {:?}", index);
            println!("clone_car: {:?}", clone_car);
            let a = clone_car.clone();
            clone_car.borrow_mut().rent = true;
            user.car = Some(a);
        } else {
            println!("Car not found");
            return;
        }
    }

    pub fn end_rental(&mut self, user: &mut User) {
        match user.car.clone() {
            Some(car) => {
                car.borrow_mut().rent = false;
                user.car = None;
            }
            None => {
                println!("User has no car");
                return;
            }
        }
    }
}
impl User {
    pub fn print_car(&self) {
        match self.car.clone() {
            Some(car) => {
                println!("{:?}", car.borrow());
            }
            None => {
                println!("User has no car");
                return;
            }
        }
    }
}




