use std::fs::File;

use polars::prelude::*;

fn example() -> DataFrame {
    let file = File::open("iris.csv").expect("could not read file");

    CsvReader::new(file)
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example());
    }
}
