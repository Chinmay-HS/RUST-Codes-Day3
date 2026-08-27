// // MEDIUM: for loop over a Vec, and over an inclusive range
// fn main() {
//     let fruits = vec!["apple", "banana", "cherry"];
//     for fruit in &fruits {
//         println!("Fruit: {}", fruit);
//     }
//     for day in 1..=7 {
//         // inclusive, goes up to and including 7
//         println!("Day {}", day);
//     }
// }

// // TINY BIT DIFFICULT: for loop with .enumerate() to get both index and value
// fn main() {
//     let scores = vec![55, 82, 91];
//     for (index, score) in scores.iter().enumerate() {
//         println!("Student {}: score {}", index + 1, score);
//     }
// }

// MEDIUM: while loop validating a condition made of two parts
fn main() {
    let mut balance = 0;
    let mut credits = 0;

    while credits < 5 {
        balance += 100;
        credits += 1;

        println!("Money follows. Money added: 100, balance now {}", balance);
    }
}

// fn main() {
//     let mut fuel = 5;
//     while fuel > 0 {
//         println!("Fuel remaining: {}", fuel);
//         fuel -= 1;
//     }
//     println!("Out of fuel!");
// }
