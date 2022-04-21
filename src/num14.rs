use crate::Solution;

#[derive(Debug)]
pub(crate) struct Solution14 { name: &'static str, difficult: u8, time: &'static str, mem: &'static str }

impl Solution14 {
    fn longest_common_prefix(&self, strs: Vec<String>) -> String {
        let mut common: String = String::new();

        let mut i:usize = 0;

        for x in strs {

            if i == 0 {
                common = x;
                i += 1;
                continue;
            }

            common = Solution14::compare_and_get_common(&common, &x);


            i += 1;
        }

        return common;
    }

    fn compare_and_get_common(str1: &String, str2: &String) -> String {
        let arr1:Vec<char> = str1.chars().collect();
        let arr2:Vec<char> = str2.chars().collect();
        let mut temp:Vec<char> = Vec::new();

        let len1: usize = arr1.len();
        if len1 == 0 { return String::new(); }
        let len2: usize = arr2.len();
        if len2 == 0 { return String::new(); }

        let mut i: usize = 0;
        loop {
            let c1:char = arr1[i];
            let c2:char = arr2[i];

            if c1 == c2 {
                temp.push(c1);
            } else {
                break;
            }

            i += 1;
            if i == len1 || i == len2 { break; }
        };

        return temp.iter().collect();
    }
}

impl Solution for Solution14 {

    fn new() -> Solution14 {
        return Solution14 { name: "Longest Common Prefix", difficult: 1, time: "0 ms", mem: "2.2 MB" }
    }

    fn print_info(&self) {
        println!("Solution: {:?}", self);
    }

    fn solve(&self) {
        let mut str_arr: Vec<String> = Vec::from([
            String::from("afi"),
            String::from("fiee"),
            String::from("free"),
            String::from("frrrrr"),
            String::from("fradsa"),

        ]);

        println!("res: {}", self.longest_common_prefix(str_arr));
    }
}

