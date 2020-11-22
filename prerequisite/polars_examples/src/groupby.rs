use polars::prelude::*;

fn example() -> Result<DataFrame> {
    let x = df!("f1" => &[1, 2, 3, 1, 3, 1], "f2" => &[1, 0, 0, 1, 2, 1])?;
    x.groupby("f1")?.select("f2").sum()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example().unwrap());
    }
}
