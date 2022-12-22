use std::{fs, error::Error};

fn main() {

    // read data in as string
    let data = fs::read_to_string("data.txt").expect("error reading file");

    // elves are split by 2 consecutive new lines
    let mut elves = data
        .split("\n\n")

        // for each elf, snacks are split by one new line
        // coerce each snack to an integer
        // sum up the amounts of the snack calories per elf
        
        .map(|elf| {
            elf
                .split("\n")
                .map(|snack| snack.parse::<i32>().expect("error parsing string to integer"))
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

        elves.sort();

        let ordered_elves = elves
            .iter()
            .rev()
            .collect::<Vec<&i32>>();

        let top_three_sum = ordered_elves[0..3].to_owned().into_iter().sum::<i32>();
        let top_elf = &ordered_elves[0];
        
        println!("top elf is {:?}, but to be safe lets get how much the top three have which is {:?}", top_elf, top_three_sum)
}
