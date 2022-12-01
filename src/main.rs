mod day01;
fn main() {
    let solutions = [day01::main];
    for (day, solution) in solutions.iter().enumerate() {
        println!("======== day {} ========", day + 1);
        solution();
    }
}
