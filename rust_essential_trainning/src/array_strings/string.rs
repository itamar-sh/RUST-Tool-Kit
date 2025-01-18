fn main() {
    let mut message = String::from("Earth");
    println!("message is {}", message);
    message.push_str(" is home.");
    println!("message is {}", message);

    // move clone copy
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);

}