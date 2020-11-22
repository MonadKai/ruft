use polars::prelude::*;

fn join() -> Result<DataFrame> {
    let temp = df!("days" => &[0, 1, 2, 3, 4],
                   "temp" => &[22.1, 19.9, 7., 2., 3.])?;
    let rain = df!("days" => &[1, 2],
                   "rain" => &[0.1, 0.2])?;

    temp.left_join(&rain, "days", "days")
}

#[cfg(test)]
mod tests {
    use super::join;

    #[test]
    fn test() {
        println!("{}", join().unwrap());
    }
}
