use std::fs;

fn main(){
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let mut counter: i32 = 0;

    for row in file_contents.lines(){
        let mut temp = row.split(",");
        let mut elf1 = temp.next().unwrap().split("-");
        let elf1_lower = elf1.next().unwrap().parse::<i32>().unwrap();
        let elf1_upper = elf1.next().unwrap().parse::<i32>().unwrap();

        let mut elf2 = temp.next().unwrap().split("-");
        let elf2_lower = elf2.next().unwrap().parse::<i32>().unwrap();
        let elf2_upper = elf2.next().unwrap().parse::<i32>().unwrap();

        if check_range_in_other(elf1_lower, elf1_upper, elf2_lower, elf2_upper){
            counter += 1
        }
    }
    println!("{}", counter);
}

fn check_range_in_other(a:i32, b:i32, x:i32, y:i32) -> bool{
    if a >= x && b <= y { return true } else if a <= x && b >= y { return true } else if a > x && a < y { return true } else if b > x && b < y { return true } else if a == x || a == y || b == x || b == y { return true } else {return false}
}