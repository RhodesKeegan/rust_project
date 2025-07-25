
use std::collections::HashMap;

const fn add(x:u32, y:u32) -> u32
{
    x+y
}


struct Number
{
    value: i32
}

impl Number
{
    fn add_other_number(&self, input_number: i32) -> i32
    {
        self.value + input_number   
    }

}

struct Square
{
    length: f32
}

impl Square
{
    fn new(desired_length: f32) -> Self
    {
        Square{length: desired_length}
    }
}

impl Number
{
    fn print(&self)
    {
        println!("hello from second impl!");
    }
}

trait Shape
{
    fn area(&self) -> f32;
}

impl Shape for Square
{
    fn area(&self) -> f32 {
        self.length * self.length
    }
}



const fn add_numbers() -> i32
{
    let x = 12;
    let number = if x < 11 {5} else {6};
    let mut counter = 0;
    let result = loop {
        counter += number;
        if counter > 3*x
        {
            break counter*x;
        }
    };
    result
}

fn print_area(shape: &impl Shape)
{
    println!("{}", shape.area());
}

fn main()
{

    let a: f32 = 5.3;
    let b: f64 = 10.5;


    let mut result: u32 = add(a as u32, b as u32);
    result+=10;

    println!("{}",result);




    let mut string: String = String::from("hello");
    let first_ref = &string;
    let second_ref = &string;

    println!("{}, {}", first_ref, second_ref);

    let third_ref = &mut string;
    *third_ref += ", world!";

    println!("{}", third_ref);


    let mut my_map = HashMap::new();
    my_map.insert(String::from("Keegan"), 24 as i32);

    println!("map value {}", my_map["Keegan"]);

    let number = Number{value: 5};
    let b = number;

    println!("The result is {}", b.value);

    println!("The result of add number is {}", add_numbers());

    println!("The result of number add is: {}", b.add_other_number(a as i32));
    b.print();



    let square = &Square::new(5 as f32);

    println!("Area of square: {}", square.area());
    print_area(square);
}