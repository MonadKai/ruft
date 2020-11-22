use polars::prelude::*;

fn example() -> Series {
    let s = Series::new("dollars", &[1, 2, 3]);
    s.eq(1).into_series()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example());
    }
}
