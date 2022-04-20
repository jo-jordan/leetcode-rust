use crate::Solution;

#[derive(Debug)]
pub(crate) struct Solution13 { name: &'static str, difficult: u8, time: &'static str, mem: &'static str }

impl Solution13 {
    pub fn roman_to_int(&self, s: String) -> i32 {
        // Special resolve while meet:
        // IV IX
        // XL XC
        // CD CM

        let array: Vec<char> = s.chars().collect();
        let len = array.len();
        let mut i: usize = 0;
        let mut sum:i32 = 0;

        loop {
            let roman: char = array[i];


            sum += match roman {
                'I' => {
                    if i + 1 != len {
                        let next_char = array[i + 1];
                        if next_char == 'V' { i += 1; 4 }
                        else if next_char == 'X' { i += 1; 9 }
                        else { 1 }
                    } else { 1 }
                },
                'V' => {
                    5
                },
                'X' => {
                    if i + 1 != len {
                        let next_char = array[i + 1];
                        if next_char == 'L' { i += 1; 40 }
                        else if next_char == 'C' { i += 1; 90 }
                        else { 10 }
                    } else { 10 }
                },
                'L' => {
                    50
                },
                'C' => {
                    if i + 1 != len {
                        let next_char = array[i + 1];
                        if next_char == 'D' { i += 1; 400 }
                        else if next_char == 'M' { i += 1; 900 }
                        else { 100 }
                    } else { 100 }
                },
                'D' => {
                    500
                },
                _ => 1000
            };

            i += 1;
            if i == array.len() { break; }
        }

        return sum;
    }
}

impl Solution for Solution13 {

    fn new() -> Solution13 {
        return Solution13 { name: "Roman to Integer", difficult: 1, time: "4 ms", mem: "2.2 MB" }
    }

    fn print_info(&self) {
        println!("Solution: {:?}", self);
    }

    fn solve(&self) {
        let str = String::from("DCXXI");

        println!("res: {}", self.roman_to_int(str));
    }
}

