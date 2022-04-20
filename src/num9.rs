use crate::Solution;

pub(crate) struct Solution9 { name: &'static str, difficult: u8 }

impl Solution for Solution9 {

    fn new() -> Solution9 {
        return Solution9 { name: "Solution9", difficult: 1 }
    }

    fn print_info(&self) {
        println!("Solution name: {}, difficult: {}", self.name, self.difficult);
    }

    fn solve(&self) {
        println!("result: {}", self.is_palindrome(121));
    }
}

impl Solution9 {
    fn is_palindrome(&self, x: i32) -> bool {
        if x < 0 { return false; }
        if x < 10 { return true; }

        let mut num = x as usize;
        let mut num_arr: [usize; 32] = [0; 32];
        let mut i: usize = 0;

        loop {
            let rem: usize = num % 10;
            num_arr[i] = rem;
            i += 1;

            num = num / 10;
            if num == 0 {
                break;
            }
        }

        let len: usize = i;
        let half_len: usize = len / 2;

        i -= 1;
        let mut tail_i = 0usize;

        loop {
            if tail_i > (half_len - 1usize) { break; }

            if num_arr[i] != num_arr[tail_i] {
                return false;
            }

            i -= 1;
            tail_i += 1;
        }

        return true;
    }
}

