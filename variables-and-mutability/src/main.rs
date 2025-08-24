const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "Spring";

    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time = "06:00";
    let event_time = 6;

    println!("{season} {points_scored} {event_time} {TOUCHDOWN_POINTS}");
    println!("{} {} {} {}", season, points_scored, event_time, TOUCHDOWN_POINTS);
    print!("{0} {1} {2} {3}", season, points_scored, event_time, TOUCHDOWN_POINTS);

    #[allow(unused_variables)]
    let favorite_beverage = "Lipton Ice Tea";
}
