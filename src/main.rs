use json::json_struct::JsonMap;
use json_parse::json_parse::json_parse;
use std::fs;
pub mod json;
pub mod json_parse;
//pub use crate::json::json_struct;
/*
fn f(data: &str) {
    for i in 0..data.len() {
        if data.chars().nth(i + 1).unwrap() == 'f'
            && data.chars().nth(i + 2).unwrap() == 'a'
            && data.chars().nth(i + 3).unwrap() == 'l'
            && data.chars().nth(i + 4).unwrap() == 's'
            && data.chars().nth(i + 5).unwrap() == 'e'
        {
            print!("GooD");
        }
    }
}
*/

fn main() {
    let mut map = JsonMap::new();
    let data = fs::read_to_string("D:\\RustJSON\\RustJSON\\target\\debug\\2.json").unwrap();

    json_parse(&data, &mut map);
    //f(&data);
    println!("Data:");
    for i in 0..map.rawdata.len() {
        println!("{}", map.rawdata[i]);
    }
}
