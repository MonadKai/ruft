use polars::prelude::*;

fn example() -> Series {
    let dates = &[
        "2020-08-21",
        "2020-08-21",
        "2020-08-22",
        "2020-08-23",
        "2020-08-22",
    ];
    let fmt = "%Y-%m-%d";
    Date32Chunked::parse_from_str_slice("date", dates, fmt).into_series()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example());
    }
}
