fn main() {
    let distance: i32 = 1_337;
    let miles: i16 = distance as i16;

    let height: f64 = 150.34546;
    println!("{height:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = true;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    let distances: [i8; 4] =  [13, 23, 75, 100];
    println!("{:#?}", distances);

    //let combo = (miles, height, is_my_type_of_coffee, distances);
    let combo: (i16, f64, bool, [i8; 4]) = (miles, height, is_my_type_of_coffee, distances);
    println!("{combo:#?}");
}
