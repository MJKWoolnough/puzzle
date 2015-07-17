trait Constraint<T> {
    fn constrain(&self, &mut [T])
}

struct Puzzle<T> {
    constraints: [Constraint<T>]
    nums: [T]
}

impl Puzzle<T> {
    fn solve(&self, &mut [T]) -> bool {
        return false
    }
}

#[test]
fn it_works() {
}
