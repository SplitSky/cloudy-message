use std::fs::File;
use std::io::Read;

fn read_log(file_path: String) -> String {
    let mut file_buff = match File::open(file_path) {
        // handle file errors
        Ok(file) => file,
        Err(error) => panic!("Issue opening file {error:?}"),
    };
    let mut file_content = String::new(); // create file buffer
    match file_buff.read_to_string(&mut file_content) {
        Ok(content) => content.to_string(),
        Err(error) => panic!("Issue parsing file content {error:?}"),
    }
}

fn main() {
    let some_log = read_log("./temp.txt".to_string());
    println!("{}", some_log);
}
