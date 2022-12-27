use std::fs;

fn main(){
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();

    let mut stack: Vec<Vec<char>> = Vec::new();
    for _nr in 0..=8{
        let inner_vec:Vec<char> = Vec::new();
        stack.push(inner_vec);
    }

    for row in (0..=8).rev(){
        let mut column_index = 0;
        for part in content_vec[row].split(']'){
            let letter = part.trim().to_string();
            //println!("{:?}{}", letter, column_index);
            if letter.len() == 2{
                stack[column_index].push(letter.chars().nth(1).unwrap());
            }

            column_index += 1;
        }
    }
    for index in 10..content_vec.len(){
        let mut cmd = content_vec[index].split(" ");
        // println!("{:?}", cmd);

        cmd.next();

        let amount: i32 = cmd.next().unwrap().parse().unwrap();
        cmd.next();
        let from: i32 = cmd.next().unwrap().parse().unwrap();
        cmd.next();
        let to: i32 = cmd.next().unwrap().parse().unwrap();

        let mut mov: Vec<char> = Vec::new();
        for _i in 0..amount{
            let c = stack[(from-1) as usize].pop().unwrap();
            mov.push(c);
        }
        for _i in 0..amount{
            let m = mov.pop().unwrap();
            stack[(to-1) as usize].push(m);
        }
    }
    for s in stack{
        println!("{:?}", s);
    }
}