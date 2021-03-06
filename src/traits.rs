trait Animal{
    fn create(name: &'static str) -> Self; // static method since it doesnt take &self as an arg

    fn name(&self) -> &'static str;

    fn talk(&self){
        println!("{} cannot talk", self.name());
    }
}


struct Human{
    name: &'static str
}


impl Animal for Human{
    fn create(name: &'static str) -> Human{
        Human{name: name}
    }
    fn name(&self) -> &'static str{
        self.name
    }
    
    fn talk(&self){
        println!("{} says hello", self.name)
    }
}


struct Cat{
    name: &'static str
}

impl Animal for Cat{
    fn create(name: &'static str) -> Cat{
        Cat{name: name}
    }

    fn name(&self) -> &'static str{
        self.name
    }
    
    fn talk(&self){
        println!("{} says hello", self.name)
    }
}

trait Summable<T>{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32{
        let mut result:i32 = 0;
        for x in self{
            result+= *x;
        }
        return result;
    }
}

fn traits(){
    // let h = Human{name: "John"};
    let h = Human::create("John");
    h.talk();

    // let c = Cat{name:"Misty"};
    let c = Cat::create("Misty");
    c.talk();

    // something cool
    // let hu = Animal::create("john"); //will throw an error coz compiler confused to choose between cat and human

    //  but if we define the type of variable it will be chosen by rust automatically
    let hu:Human = Animal::create("john");
    hu.talk();

    // You can also define traits for structs which you havent defined yourself

    let a = vec![1,2,3];
    println!("sum = {}", a.sum()) //sum is defined by me and is not a default implemented 

}

pub fn run(){
    traits()

}