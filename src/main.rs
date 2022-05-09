use std::collections::HashMap;


fn main() {
    ////// scala /////// 
    let unsigned: u8 = 10;
    let signed: i8 = -15;
    let float: f32 = 1.2;
    println!("unsigned: {} signed: {} float: {}", unsigned, signed, float);

    let letter = "c";
    let emoji = "\u{1F600}";

    println!("letter: {}, emoji: {}", letter, emoji);

    ////// array //////
    let arr: [u8; 3] = [1, 2, 3];

    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    //print the entire structure of arr
    println!("{:?}", other_arr);


    // tuple can hold elements of different types and uses dot notationk

    // can be stricly declared
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    
    // or rust can infer the types
    let tuple2 = (3, 5);

    println!("first {}, second {}, third {}",tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a, b, c) = tuple;

    //destructuring of tuple
    println!("first {}, second {}, third {}", a, b, c);

    //functon call
    println!("{}", is_even(2));

    // strings
    let mut string: String = String::from("Hello World");

    let slice = &string[..6];

    println!("the slice length is {}", slice.len());

    string.push('1');
    string.push_str("! yo");
    string = string.replace("Hello", "Later");
    
    println!("{}", string);

    //////// control flow ////////
    //if statement
    let n = 3;
    if n > 0 {
        println!("greater than 0");
    } else if n < 0 {
        println!("less than 0");
    } else {
        println!("is 0");
    }


    // for loop
    for i in 0..6 {
        println!("{}", i);
    }

    // while loop
    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i += 1;
        if i == 3 {
            println!("exit");
            break
        }
    }

    // match (similar to switch)
    let i = 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1, 2"),
        3..=4 => println!("3, 4"),
        _=> println!("default")
    }

    
    // create an instance of a struct
    // a string slice is actually needed but not allowed so a new string must be created and passed in
    let name = String::from("rocket dog");
    let dog = Dog {name: name, age: 3};

    // struct method call
    dog.bark_name();

    // trait implementation
    println!("{} {}", dog.can_fly(), dog.is_animal());

    // enum
    // call the enum class created, use of double colon to reference the field
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(3);
    let c: MyEnum = MyEnum::C{x:10, y:20};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    // another way to extract the value of the enum is to use an if statement
    if let MyEnum::B(val) = c {
        println!("{}", val);
    }

    if let MyEnum::C{x, y} = c {
        println!("{} {}", x, y);
    }

    // vector same as an array except its size can be dynamically altered
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5]; // use of a rust provided macro
    // get length
    vec.len();
    // access values by index bracket notation
    vec[0];
    // add elements
    vec.push(13);
    // remove elements
    vec.remove(0);
    // print entire vector
    println!("{:?}", vec);

    //Map objects with key value pairing
    //create a mutable map create a new instance of hashmap
    let mut map = HashMap::new();
    // insert a key and value
    map.insert(0, "Hi");
    map.insert(1, "Hi2");
    println!("{:?}", map);

    //pass in the pointer of the value
    match map.get(&0) {
        Some(str) => println!("{}", str),
        None => println!("Dosen't exist in map"),
    }

    match map.get(&2) {
        Some(str) => println!("{}", str),
        _=> println!("Dosen't exist in the map"),
    }

    map.remove(&0);
    println!("{:?}", map);



    // option
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    // use an unwrap function or the match statement method used in the hashmap
    // unwrapping a Some variant will extract the value wrapped. Returns type thats expected of the option.
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // unwrapping a None variant will panic (throw exception)
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());


    // result
    let divide = divide(4, 2);

    //could also use expect to send error if not valid value
    let res = divide.expect("we crashed");

    // match divide {
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v)
    // }

    // if divide.is_ok() {
    //     println!("{}", divide.unwrap());
    // }

    println!("{}", divide.unwrap_or(100));
}

// function definition
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0
}

// structs (similar to classes)
struct Dog {
    name: String,
    age: u8
}

// to add a method to the struct use impl keyword
impl Dog {
    fn bark_name(&self) {
        println!("{}", self.name);
    }
}


// to use the trait
impl Animal for Dog {
    fn can_fly(&self) -> bool {
        false
    }
}

// trait definition (traits are similar to interface)
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

// enum
// attribute that tells how to print this enum
#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}


// options
fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None 
    } else {
        Some(dividend / divisor)
    }
}

// result

// enum with attribute used to define an error
#[derive(Debug)]
enum MyError {
    Error1
}

// Ok value wrapper that contains a value
fn divideR(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}