
const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "Autumn";
    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time: &str  = "06:00";
    let event_time: i32  = 6;

    println!("");
    println!("season:  {season} | points_scored: {points_scored} | event_time: {event_time} | TOUCHDOWN_POINTS: {TOUCHDOWN_POINTS} ");

    println!("");
    println!("season:  {} | points_scored: {} | event_time: {} | TOUCHDOWN_POINTS: {} ", season, points_scored, event_time, TOUCHDOWN_POINTS);

    println!("");
    println!("season:  {0} | points_scored: {1} | event_time: {2} | TOUCHDOWN_POINTS: {3} ", season, points_scored, event_time, TOUCHDOWN_POINTS);

    #[allow(unused_variables)]
    let favorite_beverage: &str = "Coffee";
}
