fn main() {
    // array data
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index]);

    // multidimensional array
    let parking_lot = [[1, 2, 3],
    [4, 5, 6]];

    let number = parking_lot[1][2];
    println!("number is {}", number);

    let garage = [[[0; 10]; 10]; 100];

    // tuple data
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);
}