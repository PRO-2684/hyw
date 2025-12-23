#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use hyw_embed::add;

fn main() {
    let left = 2;
    let right = 2;
    let result = add(left, right);
    println!("The sum of {left} and {right} is {result}");
}
