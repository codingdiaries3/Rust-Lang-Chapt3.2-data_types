use std::io;

fn main() {

    /*Floating-point types*/
   let x = 2.0; //f64 (default)
   let y: f32 = 3.0; //f32

   /*Numeric Operations*/
   //addition
   let sum = 5 + 10;

   //subtraction
   let difference = 95.5 - 4.3;

   //multiplication
   let product = 4 * 30;

   //division
   let quotient = 56.7 / 32.2;

   //remainder
   let remainder = 43 % 5;

   /*Boolean Types*/
   let t = true;
   let f: bool = false; //with explicit type annotation

   /*The Character Type*/
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /* The Tuple Type*/

    //2 code example of how a tuple can have different value types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //3 code example of how you can destructure a tuple value
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y); //value the program print is 6.4

    //4 code example to access a tuple element
    let x: (i32, f64, u8) = (500, 6.4, 1 );
    let five_hundred = x.0; //0 is the index which has the 500 number
    let six_point_four = x.1; //1 is the index which has the 6.4 factoring point
    let one = x.2; //2 is the index which has the 1 number

    /* The Array Type*/
    // 2 code example of an array seperating each element with a comma
    let a = [1, 2, 3, 4, 5];

    //5 code example of how to write out an array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //6 code example to initiate an array with same element and type
    //the array named a will contain 5 elements that will all 
    //be set to the value 3 initially.
    let a = [3;5];
    //code above is the same as writing 
    let a = [3, 3, 3, 3, 3];

    //**Accessing Array Elements
    //7 code example to access an element in an array
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    //**Invalid Array Element Access
    //8 code example of invalid access to an array
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array indx.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);


}


