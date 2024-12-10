use std::fs::read_to_string;
use std::path::Path;

fn main() {
   get_total_distance();
}

fn get_total_distance() -> String {
    let path = Path::new("../inputs/day_1.txt");
    let _display = path.display();
    
    let inputs: Vec<String> = read_input(path);


    let mut distance: i32 = 0;
    // Iterate line by line, remove the whitespace and return the difference between the two values
    for i in inputs{
        let mut nums = i.split_whitespace();
        let first: i32 = nums.next().unwrap().parse().expect("This should be a number!");
        let second: i32 = nums.next().unwrap().parse().expect("This should be a number!");
       


        if first > second {
            distance = distance + (first - second)
        } else {
            distance = distance + (second - first)    
        };
    }
    println!("{}", distance);
    distance.to_string()
}


fn read_input(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()

}
