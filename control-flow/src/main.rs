fn main() {
    let color = "green";
    println!("Color {} corresponds to code {}.", color, color_to_number(color));

    let number = 5;
    println!("No recursion: !{} = {}", number, factorial_no_recursion(number));
    println!("Recursion: !{} = {}", number, factorial_with_recursion(number));
}

fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0
    }
}

fn factorial_no_recursion(number: i32) -> i32 {
    let mut result = 1;
    let mut count = number;

    while count > 1 {
        result *= count;
        count -= 1;
    }

    result
}

fn factorial_with_recursion(number: i32) -> i32 {
    if number < 2 {
        1
    } else {
        number * factorial_with_recursion(number-1)
    }
}
