
use std::fs::read_to_string;



fn main() {
    
    let filename: &str = "data.txt";
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string());
    }

    // For each line, find all numbers
    let mut num_tot = 0;
    for line in lines {
        let mut num_vec: Vec<usize> = Vec::new();
        let mut s = line.clone();

        
        loop {
            if let Some((a, b)) = find_first_number(&s) {
                num_vec.push(a);
                s = s.clone().split_off(b);
            } else {
                num_tot += 10*num_vec[0] + num_vec[num_vec.len()-1];
                break;
            }
        }
    }

    println!("{}", num_tot);

}



fn find_first_number(s: &str) -> Option<(usize, usize)> {
    let num_strings = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut str_index = s.len();
    let mut found_number_i = 0;

    for (x, n) in num_strings.iter().enumerate() {
        if let Some(i) = s.find(n) {
            if i < str_index {
                str_index = i;
                found_number_i = x;
            }
        }
    }

    if str_index == s.len() {
        None
    } else {
        Some((found_number_i%10, str_index + 1))
    }
}