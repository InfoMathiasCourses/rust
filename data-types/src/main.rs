fn main() {
    let separated: i32 = 1_000_000;

    let casted = separated as i16;

    let float_value = 3.14159;
    println!("{:.3}", float_value);

    let with_milk: bool = true;
    let with_sugar: bool = false;

    let is_my_type_of_coffee = with_milk && with_sugar;

    let is_acceptable_coffee = with_milk || with_sugar;

    let an_array = [1, 2, 3, 4];
    dbg!(an_array);

    let a_tuple = (separated, float_value, with_milk, an_array);
    dbg!(a_tuple);
}
