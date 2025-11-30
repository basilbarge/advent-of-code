use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Could not read file");

    let calories: Vec<&str> = contents.split("\n\n").collect();

    let mut calorie_count: Vec<u32> = Vec::new();

    for calorie in calories {
        let single_calories: u32 = calorie.split("\n").map(|x| match x.parse::<u32>() {
            Ok(num) => num,
            Err(_) => 0,
        }).collect::<Vec<u32>>().iter().sum();

        calorie_count.push(single_calories);
    }

    let mut max = calorie_count.iter().max().unwrap();
    let mut index = calorie_count.iter().position(|&r| &r == max).unwrap();

    println!("Largest calorie count: {max} at {index}");    

    let mut top_3: [u32;3] = [0;3];

    top_3[0] = calorie_count.swap_remove(index); 

    max = calorie_count.iter().max().unwrap();
    index = calorie_count.iter().position(|&r| &r == max).unwrap();

    top_3[1] = calorie_count.swap_remove(index); 

    max = calorie_count.iter().max().unwrap();
    index = calorie_count.iter().position(|&r| &r == max).unwrap();

    top_3[2] = calorie_count.swap_remove(index); 

    let top_3_total: u32 = top_3.iter().sum();

    println!("Total of top 3 Elves: {top_3_total}");
}

