use polars::prelude::*;

fn example() -> Series {
    let s: Series = [1, 2, 3].iter().collect();
    s.i32()
        .expect("datatype mismatch")
        .into_iter()
        .map(|ov| match ov {
            Some(v) => Some(v * v),
            None => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test() {
        println!("{}", example());
    }
}
