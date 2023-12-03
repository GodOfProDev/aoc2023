use std::fs;

fn main() {
    println!("part1");
    let file_path = "E:\\Programming\\rustProjects\\aoc2023\\day-01\\src\\bin\\input1.txt";

    let mut input = fs::read_to_string(file_path).unwrap();
    input.push('\r');

    let mut vec: Vec<Vec<char>> = Vec::new();
    let mut tec: Vec<char> = Vec::new();

    for x in input.chars() {
        if char::is_numeric(x) {
            tec.push(x);
        }

        if x == '\r' {
            vec.push(tec.clone());
            tec.clear();
        }
    }

    for y in vec.iter_mut() {
        if y.len() == 1 {
            y.push(*y.get(0).unwrap());
        }
    }

    for y in vec.iter_mut() {
        if y.len() > 2 {
            let k = y.clone();
            let x = k.get(0).unwrap();
            let z = k.get(y.len() - 1).unwrap();

            y.clear();

            y.push(*x);
            y.push(*z);
        }
    }

    let mut sum: u32 = 0;

    for j in vec.iter() {
        let str: String = j.into_iter().collect();
        let strint = str.parse::<u32>().unwrap();
        sum += strint;
    }

    dbg!(sum);
}
