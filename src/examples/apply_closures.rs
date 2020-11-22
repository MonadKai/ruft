use polars::prelude::*;

fn example() -> Series {
    let s: Series = Series::new("values", [Some(1.0), None, Some(3.0)]);
    s.f64().unwrap().apply(|v| v.powf(2.0)).into_series()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example());
    }
}
