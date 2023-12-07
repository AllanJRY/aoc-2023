fn main() {
    let input = include_str!("../../input2.txt");
    let value = day_01::part2::process(input).unwrap();
    println!("{value}");
}
