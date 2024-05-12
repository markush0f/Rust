use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");

    // Variables
    let mut my_string: &str = "My first String!!";
    println!("{my_string}");
    my_string = "We changed the string!!";
    println!("{my_string}");

    let my_string2: String = String::from("This type is with String");
    println!("{my_string2}");

    let mut my_int: i32 = 7;
    my_int = my_int + 4;
    print!("This is my first Integer: {}", my_int);

    let my_float: f64 = 6.5;
    println!("This is my first Float{my_float}");

    let my_boolean: bool = false;
    println!("This is my fisrt Boolean: {my_boolean}");

    // Constants
    const MY_CONST: &str = "My first const";
    println!("{MY_CONST}");

    // if
    if my_int == 11 {
        println!("Int value: 11");
    } else if my_int == 12 {
        println!("Int value: 12");
    } else {
        println!("Int value: unknow")
    }

    // List
    let mut my_list: Vec<&str> = vec!["Markus", "Abramian", "Medina"];
    my_list.push("19");
    println!("My first List: {:?}", my_list);

    // Sets
    let mut my_set: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    // into_iter: converts the vector an iterator
    // collect:
    my_set.insert(1);
    println!("My first Set: {:?}", my_set);

    // Maps
    let mut my_map: HashMap<&str, &str> = vec![("Name", "Markus"), ("Surname", "Abramian")]
        .into_iter()
        .collect();
    my_map.insert("Age", "19");
    println!("My first Map: {:?}", my_map);

    // Loops
    print!("My first Loop For: ");
    for value in &my_list {
        print!("{value} ")
    }

    println!("\nLoop for with map: ");
    for (key, value) in &my_map {
        println!("Key: {} Value: {}", key, value)
    }
    print!("Loop while: ");
    let mut my_counter = 0;
    while my_counter < my_list.len() {
        print!("{} ", my_list[my_counter]);
        my_counter += 1;
    }
    // Functions
    my_function();

    // Structures
    let my_struct: MyStruct = MyStruct::new("Markus", 19);
    println!("Name: {}, Age: {}", my_struct.name, my_struct.age)
}

fn my_function() {
    println!("\nMy first function")
}

//  Structures
struct MyStruct {
    name: String,
    age: i32,
}

impl MyStruct {
    fn new(name: &str, age: i32) -> MyStruct {
        MyStruct {
            name: String::from(name),
            age,
        }
    }
}
