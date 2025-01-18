fn main() {
    println!("Hello, world!");
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);
    let mut x: u8 = 255;
    x = x + 1;
    println!("x is {}", x);
    let x: f32 = 10.123456789123456789;
    println!("x is {}", x);
    let a = 10;
    let b = 3.0;
    let c = a as f64 / (b + 1.0);
    println!("c is {}", c);

    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    print!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);

    // bitwise operation
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = !value; // NOT
    println!("value is {:08b}", value);

    value = value & 0b1111_0111; // clear bit with AND
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value | 0b0100_0000; // set bit with OR
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101; // XOR
    println!("value is {:08b}", value);

    value = value << 4; // shift left by 4
    println!("value is {:08b}", value);

    value = value >> 2; // shift right by 2
    println!("value is {:08b}", value);

    // boolean data
    let a = true;
    let b = false;
    println!("a is {} and b is {}");
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b {}", a ^ b);

    let c = (a ^ b) && panic!();
    println!("c is {}", c);

    // comparison operators
    let a = 1;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);

    // char data
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);

    // average
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");

    // shadowing
    let planet = "Earth";
    {
        println!("planet is {}", planet);
        let mut planet = 4;
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);

    // if let
    let number = None;
    if let Some(13) = number {
        println!("thirteen");
    }
}
