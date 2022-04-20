mod num9;
mod num13;

trait Solution {
    fn new() -> Self;

    fn print_info(&self);

    fn solve(&self);
}

fn main() {
    let solution9: num9::Solution9 = Solution::new();

    solution9.print_info();
    solution9.solve();


}
