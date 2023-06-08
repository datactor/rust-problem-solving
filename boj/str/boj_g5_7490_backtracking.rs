// https://www.acmicpc.net/problem/7490

#![allow(non_snake_case)]

use std::{
    io::{self, BufRead, BufReader},
};

macro_rules! read_num {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.trim().parse::<$type>().expect("Failed to parse")
        }
    }
}

macro_rules! Ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut buf_to_string = String::new();

            let t = read_num!(read_buf, buf_to_string, usize);

            for _ in 0..t {
                let n = read_num!(read_buf, buf_to_string, i32);
                find_ex(0, 1, 1, 1, "1".to_string(), n);

                println!();
            }

            Ok(())
        }
    }
}

fn find_ex(sum: i32, sign: i32, num: i32, n: i32, str: String, n_max: i32) {
    if n == n_max {
        let sum = sum + (num * sign);
        if sum == 0 {
            println!("{}", str);
        }
    } else {
        find_ex(sum, sign, num * 10 + (n + 1), n + 1, format!("{} {}", str, (n + 1)), n_max);
        find_ex(sum + (sign * num), 1, n + 1, n + 1, format!("{}+{}", str, (n + 1)), n_max);
        find_ex(sum + (sign * num), -1, n + 1, n + 1, format!("{}-{}", str, (n + 1)), n_max);
    }
}

fn main() -> io::Result<()> {
    Ok!(())
}