
trait Dog {
    fn bark(&self);
    fn walk(&self);
    fn pee(&self) {

    }
}

trait Cat {
    fn meow(&self);
    fn groom(&self);
    fn introduce(&self) -> Self where Self: Sized;
}

struct Poodle {
    name: String,
}

impl Poodle {
    pub fn new(name: &str) -> Self {
        Poodle { name: String::from(name) }
    }
}

impl Dog for u8 {
    fn bark(&self) {
        println!("{} BARK", self);
    }

    fn pee(&self) {
        println!("{} PEE", self);
    }

    fn walk(&self) {
        println!("{} WALK", self);
    }
}


impl Dog for Poodle {
    fn bark(&self) {
        println!("{}: BARK!", self.name);
    }

    fn walk(&self) {
        println!("{}: WALK", self.name);
    }

    fn pee(&self) {
        println!("{}: PEE", self.name);
    }
}


struct MutantDog {
    name: String,
}

impl Dog for MutantDog {
    fn bark(&self) {
        println!("{}: Mutant BARK!", self.name);
    }

    fn walk(&self) {
        println!("{}: Mutant WALK", self.name);
    }

    fn pee(&self) {
        println!("{}: Mutant PEE", self.name);
    }
}

impl Cat for MutantDog {
 
    fn groom(&self) {
       println!("{}: Mutant GROOM", self.name);        
    }
    fn meow(&self) {
       println!("{}: Mutant MEOW", self.name);
    }

    fn introduce(&self) -> Self where Self: Sized {
        MutantDog { name: String::from(&self.name) }
    }

}

impl MutantDog {
    pub fn new(name: &str) -> Self {
        MutantDog { name: String::from(name) }
    }
}


struct Zoo {
    dogs: Vec<Box<dyn Dog>>,
    cats: Vec<Box<dyn Cat>>
}

impl Zoo {
    pub fn new(dogs: Vec<Box<dyn Dog>>, cats: Vec<Box<dyn Cat>>) -> Self {
        Zoo { dogs, cats }
    }

    pub fn walk_around(&self) {
        for dog in &self.dogs {
            dog.walk();
        }
    }
}

fn main() {
    let zoo = Zoo::new(vec![Box::new(Poodle::new("james")), Box::new(MutantDog::new("Charles")), Box::new(0u8)], vec![Box::new(MutantDog::new("james"))]);
    zoo.walk_around();
}
