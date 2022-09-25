fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_two = 3;

    if number_two != 0 {
        println!("number was something other than zero");
    }

    let number_three = 6;

    if number_three % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_three % 3 == 0 {
        println!("number is divisible by 3");
    } else if number_three % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");
}
