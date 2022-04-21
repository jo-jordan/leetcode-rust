mod num9;
mod num13;
mod num14;

trait Solution {
    fn new() -> Self;

    fn print_info(&self);

    fn solve(&self);
}

fn main() {
    // let solution9: num9::Solution9 = Solution::new();
    // let solution13: num13::Solution13 = Solution::new();
    let solution14: num14::Solution14 = Solution::new();
    //
    // solution9.print_info();
    // solution9.solve();
    //
    // solution13.print_info();
    // solution13.solve();
    //
    solution14.print_info();
    solution14.solve();

}
