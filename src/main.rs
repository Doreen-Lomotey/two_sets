use std::io;

fn main() {
    // Read the number n
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();

    let n: u64 = text.trim().parse().unwrap();

    // Formula for total sum from 1 to n
    let total_sum = n * (n + 1) / 2;

    // If odd, impossible to split equally
    if total_sum % 2 != 0 {
        println!("NO");
        return;
    }

    println!("YES");

    let mut needed = total_sum / 2;

    let mut first_group: Vec<u64> = Vec::new();
    let mut second_group: Vec<u64> = Vec::new();

    // Start from biggest number downwards
    for value in (1..=n).rev() {
        if value <= needed {
            first_group.push(value);
            needed -= value;
        } else {
            second_group.push(value);
        }
    }

    // Print first set
    println!("{}", first_group.len());

    for num in &first_group {
        print!("{} ", num);
    }

    println!();

    // Print second set
    println!("{}", second_group.len());

    for num in &second_group {
        print!("{} ", num);
    }

    println!();
}