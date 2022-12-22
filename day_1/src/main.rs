use std::{fs, error::Error};

fn main() {

    // read data in as string
    let data = fs::read_to_string("data.txt").expect("error reading file");

    // elves are split by 2 consecutive new lines
    let calorie_rich_elf = data
        .split("\n\n")

        // for each elf, snacks are split by one new line
        // coerce each snack to an integer
        // sum up the amounts of the snack calories per elf
        // grab the elf with the most calories
        .map(|elf| {
            elf
                .split("\n")
                .map(|snack| snack.parse::<i32>().expect("error parsing string to integer"))
                .sum::<i32>()
        })
        .max().expect("no values available to reduce to a max");


        println!("{:?}", calorie_rich_elf)
}
