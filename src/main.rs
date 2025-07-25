
const fn add(x:u32, y:u32) -> u32
{
    x+y
}


struct Number
{
    value: i32
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




    let number = Number{value: 5};
    let b = number;

    println!("The result is {}", b.value);

    println!("The result of add number is {}", add_numbers());
}