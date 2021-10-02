use crate::structs::PassportBuilder;

pub mod passport_builder;
pub mod structs;

fn main() {
    let results = include_str!("input.txt")
        .split("\n\n")
        .map(PassportBuilder::parse)
        .map(PassportBuilder::build);

    let num_valid = results.filter(Result::is_ok).count();
    println!("{} passport records were valid", num_valid);
}
