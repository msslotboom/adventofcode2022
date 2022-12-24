use std::fs;

fn main() {
    let file_contents = fs::read_to_string("test_data.txt").unwrap();

    let mut sum_list: Vec<i32> = Vec::new();
    let mut current_sum = 0;

    for row in file_contents.lines(){
        if row == ""{
            sum_list.push(current_sum);
            current_sum = 0;
        }
        else{
            current_sum += row.parse::<i32>().unwrap();
        }
    }
    sum_list.push(current_sum);

    sum_list.sort();

    let mut top3 = 0;
    for _i in 1..=3{
        top3 += sum_list.last().unwrap();
        sum_list.pop();
    }
    println!("top 3 =  {}", top3)
}