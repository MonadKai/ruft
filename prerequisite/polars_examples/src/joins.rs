use polars::prelude::*;

fn example() -> DataFrame {
    let temp = df!("days" => &[0, 1, 2, 3, 4],
                   "temp" => &[22.1, 19.9, 7., 2., 3.])
    .unwrap();
    let rain = df!("days" => &[1, 2],
                   "rain" => &[0.1, 0.2])
    .unwrap();

    temp.left_join(&rain, "days", "days").unwrap()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example());
    }
}
