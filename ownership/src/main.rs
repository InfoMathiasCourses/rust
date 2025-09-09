fn main() {

    /* Coding Challenge #1 */

    let is_concert: bool = true;
    let is_event = is_concert;

    println!("{} {}", is_concert, is_event);

    let sushi: &str = "Salmon";
    let dinner = sushi;

    println!("{} {}", sushi, dinner);

    let sushi_2: String = String::from("Salmon");
    let mut dinner_2: String = sushi_2;

    // dinner_2 now owns the value from sushi_2 therefore sushi_2
    println!("{}", dinner_2);

    eat_meal(&mut dinner_2);

    println!("{dinner_2}");


    /* Coding Challenge #2 */

    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push_str(".");
    show_itinerary(&trip);
}

fn eat_meal (meal: &mut String) {
    String::clear(meal);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(text: &mut String) {
    text.push_str("Philadelphia");
}

fn visit_new_york(text: &mut String) {
    text.push_str("New York");
}

fn visit_boston(text: &mut String){
    text.push_str("Boston");
}

fn show_itinerary(text: &String){
    println!("{text}");
}
