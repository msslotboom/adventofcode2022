use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let mut total_points = 0;

    for row in file_contents.lines(){
        let mut rps = row.split(' ');
        let elf = rps.next().unwrap();
        let wished_result = rps.next().unwrap();

        total_points += points(elf, eval_move(elf, wished_result));
    }
    println!("{}", total_points);
}

fn points(x: &str, y: &str) -> i32 {
    let mut points = 0;
    points = if y == "X" { 1 } else if y == "Y" { 2 } else { 3 };
    
    if x == "A" {
        points += if y == "Y" { 6 } else if y == "X" { 3 } else { 0 };
    }
    else if x == "B" {
        points += if y == "X" { 0 } else if y == "Y" { 3 } else { 6 };
    }
    else if x == "C" {
        points += if y == "X" { 6 } else if y == "Z" { 3 } else { 0 };
    }
    return points
}

fn eval_move<'life>(x: &str, y:&str) -> &'life str {
    if x == "A" {
        return if y == "Z" { "Y" } else if y == "X" { "Z" } else { "X" };
    }
    else if x == "B" {
        return if y == "Z" { "Z" } else if y == "X" { "X" } else { "Y" };
    }
    else if x == "C" {
        return if y == "Z" { "X" } else if y == "X" { "Y" } else { "Z" };
    }
    else{
        return ""
    }
}