use polars::prelude::*;

fn example() -> Series {
    let s = Series::new("foo", [1, 2, 3]);

    &s * &s
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example());
    }
}
