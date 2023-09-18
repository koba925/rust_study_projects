// fn fizzbuzz(end: i32) {
//     let mut x = 1;

//     while x <= end {
//         if x % 3 == 0 && x % 5 == 0 {
//             println!("{} FizzBuzz", x);
//         } else if x % 3 == 0 {
//             println!("{} Fizz", x);
//         } else if x % 5 == 0 {
//             println!("{} Buzz", x);
//         } else {
//             println!("{}", x);
//         }

//         x += 1;
//     }
// }

// fn fizzbuzz(end: i32) {
//     for x in 1..=end {
//         match x % 15 {
//             0 => println!("{} FizzBuzz", x),
//             3 | 6 | 9 | 12 => println!("{} Fizz", x),
//             5 | 10 => println!("{} Buzz", x),
//             _ => println!("{}", x),
//         }
//     }
// }

fn fizzbuzz(end: i32) {
    for x in 1..=end {
        match (x % 3, x % 5) {
            (0, 0) => println!("{} FizzBuzz", x),
            (0, _) => println!("{} Fizz", x),
            (_, 0) => println!("{} Buzz", x),
            _ => println!("{}", x),
        }
    }
}

fn main() {
    fizzbuzz(30)
}
