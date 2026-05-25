pub fn solve(n: u64) -> bool {
    let total = n * (n + 1) / 2;

    if total % 2 != 0 {
        return false;
    }

    let target = total / 2;

    let numbers: Vec<u64> = (1..=n).collect();

    // Deliberately slower repeated scanning
    let mut current = 0;

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            current += numbers[j];

            if current == target {
                return true;
            }

            if current > target {
                current = 0;
                break;
            }
        }
    }

    false
}