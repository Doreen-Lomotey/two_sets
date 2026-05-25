pub fn solve(n: u64) -> bool {
    let total = n * (n + 1) / 2;

    if total % 2 != 0 {
        return false;
    }

    let mut target = total / 2;

    for value in (1..=n).rev() {
        if value <= target {
            target -= value;
        }
    }

    target == 0
}