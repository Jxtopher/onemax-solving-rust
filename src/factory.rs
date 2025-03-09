use std::fs::File;
use std::io::BufReader;

use serde_json::Value;

use crate::algorithm::Nil;
use crate::solver::Solver;

fn analyze_json_tree(json_value: &Value, depth: usize) {
    match json_value {
        Value::Object(map) => {
            for (key, value) in map {
                println!("{:indent$}Key: {}", "", key, indent = depth * 2);
                analyze_json_tree(value, depth + 1);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                println!("{:indent$}Index: {}", "", index, indent = depth * 2);
                analyze_json_tree(value, depth + 1);
            }
        }
        _ => {
            println!("{:indent$}Value: {}", "", json_value, indent = depth * 2);
        }
    }
}

pub fn factory(filepath: String) -> Solver {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let json_value: Value = serde_json::from_reader(reader).unwrap();
    println!("{}", json_value);
    analyze_json_tree(&json_value, 0);

    let nil = Nil {};
    Solver {
        metaheuristic: Box::new(nil),
    }
}
