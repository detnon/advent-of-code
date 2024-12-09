use std::fs::read_to_string;
use std::path::Path;

fn main() {
    get_total_distance();
}

fn get_total_distance(){
    let path = Path::new("./inputs/day_1.txt");
    let _display = path.display();
    
    let _inputs: Vec<String> = read_input(path);

}


fn read_input(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()

}