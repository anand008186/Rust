fn main(){
// VARIABLES 
// can be immutable and mutable
// immutable variables are declared using let keyword
// mutable variables are declared using let mut keyword
// variables are type annotated optionally

    let x = 5;  //immutable variable
    println!("The value of x is: {}", x);
    let mut y = 10; //mutable variable
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is: {}", y);

// CONSTANTS
// immutable and cannot be declared as mutable
// constants are declared using const and type annotation is mandatory

    const NUM: i8 = 5; //constant
    const EVEN_NUM: i32 = 10; 
    println!("The value of NUM NUM_EVEN is: {} {}", NUM, EVEN_NUM);


// FUNCTIONS
// declared using fn keyword
// parameters are type annotated
// return type is also type annotated
// return keyword is used to return a value from a function

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    println!("The sum of 5 and 10 is: {}", add(5, 10));

// SIGNED AND UNSIGNED INTEGERS
// signed integers can be positive, negative or zero
// unsigned integers are always positive or zero
// signed integers are of type i8, i16, i32, i64, i128, isize
// unsigned integers are of type u8, u16, u32, u64, u128, usize

    let signed_integer: i8 = -5;
    let unsigned_integer: u8 = 5;
    println!("The signed integer is: {}", signed_integer);
    println!("The unsigned integer is: {}", unsigned_integer);
    
// DATA TYPES 
// Scalar types: integers, floating point numbers, booleans, characters
    let x:u8 = 23;
    let y:f32 = 23.0;
    let z:bool = true;
    let a:char = 'a';
    println!("x is {} y is {} z is {} a is {}", x, y, z, a); 

// Compound types: tuples, arrays
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {} y is: {} z is: {}", x, y, z);
    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr is: {}", arr[0]);

// CONTROL FLOW
 //if else statement
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

// loop statement
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

// SHADOWING
// shadowing is different from mutability
// shadowing is done by using the let keyword again
// shadowing is useful when we want to change the value and type of the variable multiple times in a single block

    let x = 5;
    println!("The value of x is: {}", x);
    let x: f32 = 3.4;
    println!("The value of x is: {}", x);

}