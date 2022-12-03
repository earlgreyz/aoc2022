use std::cmp;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut max_calories: i32 = 0;
    let mut current_calories: i32 = 0;

    while let Some(Ok(line)) = lines.next() {
        if "".eq(&line) {
            current_calories = 0 
        } else {
            let calories: i32 = line.parse().unwrap();
            current_calories += calories;
            max_calories = cmp::max(current_calories, max_calories)
        }
    }
    println!("{}", max_calories);
    Ok(())
}
