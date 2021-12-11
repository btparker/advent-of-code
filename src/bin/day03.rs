use ndarray::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

const RADIX: u32 = 10;
type KVPair = (u32, usize);

fn from_rows_to_array<T: Clone>(v: Vec<Vec<T>>) -> Array2<T> {
    if v.is_empty() {
        return Array2::from_shape_vec((0, 0), Vec::new()).unwrap();
    }
    let nrows = v.len();
    let ncols = v[0].len();
    let mut data = Vec::with_capacity(nrows * ncols);
    for row in &v {
        assert_eq!(row.len(), ncols);
        data.extend_from_slice(&row);
    }
    Array2::from_shape_vec((nrows, ncols), data).unwrap()
}

fn get_hash_count(v: Vec<u32>) -> HashMap::<u32, usize>{
    return v
        .iter()
        .fold(HashMap::<u32, usize>::new(), |mut m, x| {
            let counter = m.entry(*x).or_default();
            *counter += 1;
            m
        });
}

fn get_most_common_element(v: Vec<u32>) -> u32{
    return sort_count(v)
        .last()
        .map(|(k, _)| *k)
        .unwrap();
}

fn sort_count(v: Vec<u32>) -> Vec<KVPair>{
    let hash_count: HashMap::<u32, usize> = get_hash_count(v);
    let mut hash_count_pairs: Vec<_> = hash_count.into_iter().collect();
    hash_count_pairs.sort_by(|(ka, va): &KVPair, (kb, vb): &KVPair| {
        if va == vb { ka.cmp(&kb) } else { va.cmp(&vb) }
    });
    return hash_count_pairs;
}

fn get_least_common_element(v: Vec<u32>) -> u32{
    return sort_count(v)
        .first()
        .map(|(k, _)| *k)
        .unwrap();
}

fn load_from_file(path: PathBuf) -> Vec<Vec<u32>>{
    let file = File::open(&path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line|{
            line.expect("Invalid file line")
        })
        .map(|s|{
            s.chars()
                .map(|c| {
                    c.to_digit(RADIX).unwrap()
                })
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();
}

fn vec_numbers_to_str(num_vec: Vec<u32>)-> String {
    return num_vec.iter().map( |v| v.to_string()).collect::<String>()
}

fn get_columns(arr: Array2<u32>) -> Vec<Vec<u32>>{
    return get_axis(arr, 0);
}

fn get_rows(arr: Array2<u32>) -> Vec<Vec<u32>>{
    return get_axis(arr, 1);
}

fn get_axis(arr: Array2<u32>, axis: usize) -> Vec<Vec<u32>>{
    return arr
        .map_axis(
            Axis(axis),
            |view| view
        )
        .map(|column| column.iter().cloned().collect::<Vec<u32>>())
        .iter()
        .cloned()
        .collect::<Vec<Vec<u32>>>();
}

fn decimal_from_binary_string(vals_str: String) -> u32 {
    return isize::from_str_radix(&vals_str, 2).unwrap() as u32;
}

fn decimal_from_binary_vec(v: Vec<u32>) -> u32 {
    let str = vec_numbers_to_str(v);
    return decimal_from_binary_string(str);
}

fn part_one(data_arr:  Array2<u32>){
    println!("Part One Result:");

    let columns: Vec<Vec<u32>> = get_columns(data_arr);

    let most_common_vals = columns
        .clone()
        .into_iter()
        .map(get_most_common_element)
        .collect::<Vec<u32>>();

    let least_common_vals = columns
        .clone()
        .into_iter()
        .map(get_least_common_element)
        .collect::<Vec<u32>>();

    let gamma = decimal_from_binary_vec(most_common_vals);
    let epsilon = decimal_from_binary_vec(least_common_vals);

    println!("gamma = {:?}", gamma);
    println!("epsilon = {:?}", epsilon);
    println!("result = {:?}", gamma * epsilon);
}

fn get_column(data_arr:  Array2<u32>, ci: usize) -> Vec<u32>{
    return data_arr.slice(s![.., ci]).iter().cloned().collect::<Vec<u32>>();
}

fn get_row(data_arr:  Array2<u32>, ri: usize) -> Vec<u32>{
    return data_arr.slice(s![ri, ..]).iter().cloned().collect::<Vec<u32>>();
}

fn get_num_columns(data_arr:  Array2<u32>) -> usize {
    return data_arr.len_of(Axis(1));
}

fn get_num_rows(data_arr:  Array2<u32>) -> usize {
    return data_arr.len_of(Axis(0));
}

fn get_oxygen_generator_rating(data_arr:  Array2<u32>)-> u32 {
    let ncols = get_num_columns(data_arr.clone());

    let mut ogr_arr: Array2<u32> = data_arr.clone();
    let ogr_arr = (0 .. ncols)
        .fold(ogr_arr, |acc_arr, ci|{
            // If down to the last row, return
            if get_num_rows(acc_arr.clone()) == 1 {
                return acc_arr;
            }

            let column = get_column(acc_arr.clone(), ci);
            let most_common_element = get_most_common_element(column);
            let rows = get_rows(acc_arr.clone())
                .into_iter()
                .filter(|row|{
                    return row[ci] == most_common_element;
                })
                .collect::<Vec<Vec<u32>>>();

            return from_rows_to_array(rows);
        });

    return decimal_from_binary_vec(get_row(ogr_arr, 0));
}

fn get_co2_scrubber_rating(data_arr:  Array2<u32>)-> u32 {
    let ncols = get_num_columns(data_arr.clone());

    let mut ogr_arr: Array2<u32> = data_arr.clone();
    let ogr_arr = (0 .. ncols)
        .fold(ogr_arr, |acc_arr, ci|{
            // If down to the last row, return
            if get_num_rows(acc_arr.clone()) == 1 {
                return acc_arr;
            }

            let column = get_column(acc_arr.clone(), ci);
            let least_common_element = get_least_common_element(column);
            let rows = get_rows(acc_arr.clone())
                .into_iter()
                .filter(|row|{
                    return row[ci] == least_common_element;
                })
                .collect::<Vec<Vec<u32>>>();

            return from_rows_to_array(rows);
        });

    return decimal_from_binary_vec(get_row(ogr_arr, 0));
}

fn part_two(data_arr:  Array2<u32>) {
    // Need oxygen generator rating by the CO2 scrubber rating to determine life support rating
    let oxygen_generator_rating = get_oxygen_generator_rating(data_arr.clone());
    let co2_scrubber_rating = get_co2_scrubber_rating(data_arr.clone());

    println!("Part Two Result:");
    println!("oxygen_generator_rating {:?}", oxygen_generator_rating);
    println!("co2_scrubber_rating {:?}", co2_scrubber_rating);
    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;
    println!("life_support_rating {:?}", life_support_rating);

}

fn run(path: PathBuf){
    let data_vecs = load_from_file(path);
    let data_arr = from_rows_to_array(data_vecs);


    part_one(data_arr.clone());
    part_two(data_arr.clone());
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let input_path = PathBuf::from(&args[1]);
    println!("-- Running Day03 on {:?} --", input_path);
    run(input_path);
}
