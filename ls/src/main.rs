use std::env;
mod ls;

fn main() {
    let path: String;
    let arg_path = env::args().nth(1);

    if arg_path.is_some() {
        path = arg_path.unwrap();
    }
    else {
        path = ".".to_string();
    }
    
    if let Ok(val) = ls::list_storage(&path) {
        for value in val {
            println!("{}", value);
        }
    }
    else {
        println!("Path does not exist");
    }
}
