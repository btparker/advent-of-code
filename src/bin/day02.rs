use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use enum_map::{Enum};

#[derive(Debug, Enum)]
enum Direction {
    Forward,
    Down,
    Up,
    Error,
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    magnitude: i32,
}

#[derive(Debug)]
struct SubmarineState {
    position: i32,
    depth: i32,
    aim: i32,
}

fn build_movement(split: Vec<String>) -> Movement {
    let direction_str = String::from(split[0].to_owned());
    let magnitude = split[1].parse::<i32>().unwrap();
    let direction: Direction = match direction_str.as_ref() {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => Direction::Error,
    };


    return Movement {
        direction: direction,
        magnitude: magnitude,
    };
}

fn split_str_to_vec_str(str: String) -> Vec<String>{
    return str
        .split_whitespace()
        .map(|s|{
            s.to_owned()
        })
        .collect::<Vec<String>>();
}

fn load_from_file(path: PathBuf) -> Vec<Movement>{
    let file = File::open(&path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line|{
            line.expect("Invalid file line")
        })
        .map(split_str_to_vec_str)
        .map(build_movement)
        .collect();
}

fn apply_movement_part_one(mut sub_state: SubmarineState, movement: Movement) -> SubmarineState {
    sub_state.position += match movement.direction {
        Direction::Forward => movement.magnitude,
        _ => 0,
    };
    sub_state.depth += match movement.direction {
        Direction::Down => movement.magnitude,
        Direction::Up => -1 * movement.magnitude,
        _ => 0,
    };
    return sub_state;
}

fn apply_movement_part_two(mut sub_state: SubmarineState, movement: Movement) -> SubmarineState {
    if matches!(movement.direction, Direction::Up) {
        sub_state.aim -= movement.magnitude;
    }
    else if matches!(movement.direction, Direction::Down) {
        sub_state.aim += movement.magnitude;
    }
    else if matches!(movement.direction, Direction::Forward) {
        sub_state.position += movement.magnitude;
        sub_state.depth += sub_state.aim * movement.magnitude;
    }

    return sub_state;
}

fn part_one(movements: Vec<Movement>) {
    println!("Part One Result:");
    let sub_state = SubmarineState {
        position: 0,
        depth: 0,
        aim: 0,
    };
    let result_sub_state: SubmarineState = movements
        .into_iter()
        .fold(sub_state, apply_movement_part_one);
    let result = result_sub_state.position * result_sub_state.depth;
    println!("{:?}", result);
}

fn part_two(movements: Vec<Movement>) {
    println!("Part Two Result:");
    let sub_state = SubmarineState {
        position: 0,
        depth: 0,
        aim: 0,
    };
    let result_sub_state: SubmarineState = movements
        .into_iter()
        .fold(sub_state, apply_movement_part_two);
    let result = result_sub_state.position * result_sub_state.depth;
    println!("{:?}", result);
}

fn run(path: PathBuf){
    let movements_part_a = load_from_file(path.clone());
    part_one(movements_part_a);

    let movements_part_b = load_from_file(path.clone());
    part_two(movements_part_b);
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let input_path = PathBuf::from(&args[1]);
    println!("-- Running Day02 on {:?} --", input_path);
    run(input_path);
}
