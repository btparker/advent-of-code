use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn load_from_file(path: PathBuf) -> Vec<i64>{
    let file = File::open(&path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<i64>()
                .unwrap()
            }
        )
        .collect();
}

fn compute_number_increases(numbers: Vec<i64>) -> Vec<bool>{
    return numbers
        .windows(2) // overlapping pairs
        .map(|x| x[1] > x[0]) // is the following value greater than the previous?
        .collect();
}

fn sum_true(values: Vec<bool>) -> i64{
    return values
        .iter()
        .map(|&x| x as i64) // casting boolean to integer
        .sum();
}

fn part_one(depth_measurements: Vec<i64>) {
    // Create a vector of bools, designating if depth 'increased' or not.
    let depth_increases: Vec<bool> = compute_number_increases(depth_measurements);
    let result = sum_true(depth_increases);

    println!("Part One Result:");
    println!("{:?}", result);
}

fn part_two(depth_measurements: Vec<i64>) {
    // Create a vector of window sums
    let depth_window_sums: Vec<i64> = depth_measurements
        .windows(3) // overlapping window
        .map(|x| x.iter().sum()) // Sum the windows
        .collect();

    let depth_increases = compute_number_increases(depth_window_sums);
    let result = sum_true(depth_increases);

    println!("Part Two Result:");
    println!("{:?}", result);
}

fn run(path: PathBuf){
    let depth_measurements: Vec<i64> = load_from_file(path);
    part_one(depth_measurements.clone());
    part_two(depth_measurements.clone());
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let input_path = PathBuf::from(&args[1]);
    println!("-- Running Day01 on {:?} --", input_path);
    run(input_path);
}
