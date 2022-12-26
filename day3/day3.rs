use std::fs;
use std::collections::HashSet;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();
    let mut total_score = 0;
    let amount_lines = file_contents.lines().count();

    for index in (0..amount_lines).step_by(3){
        let sack1: HashSet<char> = content_vec[index].chars().collect();
        let sack2: HashSet<char> = content_vec[index+1].chars().collect();
        let sack3: HashSet<char> = content_vec[index+2].chars().collect();
        
        let intersection: HashSet<char> = sack1.intersection(&sack2).cloned().collect();
        let intersection: HashSet<char> = intersection.intersection(&sack3).cloned().collect();

        total_score += score(intersection.into_iter().next().unwrap());
    }
    println!("{:?}", total_score);
}

fn score(x:char)-> i32 {
    let score = x as u8;
    if score >= b'a' && score <= b'z' {
        score as i32 - b'a' as i32 + 1
    }
    else if score >= b'A' && score <= b'Z' {
        score as i32 - b'A' as i32 + 27
    }
    else {
        0
    }
}