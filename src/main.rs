fn main() {
    println!("Hello, world!");
    let mut array = [0; 3];

    array[1] = 1;
    array[2] = 2;

    for x in array {
        print!("N: {x} ");
    }
}
