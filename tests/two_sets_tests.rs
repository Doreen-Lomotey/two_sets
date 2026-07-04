use two_sets::{fast_solution, slow_solution};

#[test]
fn small_case_true() {
    // Example where partition is possible
    assert_eq!(fast_solution::solve(7), true);
    assert_eq!(slow_solution::solve(7), true);
}

#[test]
fn small_case_false() {
    // Example where partition is not possible
    assert_eq!(fast_solution::solve(3), false);
    assert_eq!(slow_solution::solve(3), false);
}

#[test]
fn consistency_check() {
    // Both implementations must always agree
    for n in 1..50 {
        assert_eq!(
            fast_solution::solve(n),
            slow_solution::solve(n)
        );
    }
}