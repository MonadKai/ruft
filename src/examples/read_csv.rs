use std::fs::File;

use polars::prelude::*;

fn example() -> Result<DataFrame> {
    let file = File::open("iris.csv").expect("could not read file");

    CsvReader::new(file)
        .infer_schema(None)
        .has_header(true)
        .finish()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        example();
    }
}
