#[derive(Debug, Default)]
#[allow(dead_code)]
enum ChocolateFlavor {
    #[default]
    Milk,
    Dark,
    White,
    Hazelnut,
    Mint,
}

fn prize(choco: ChocolateFlavor) -> f32 {
    match choco {
        ChocolateFlavor::Milk | ChocolateFlavor::Dark | ChocolateFlavor::White => 2.49,
        ChocolateFlavor::Hazelnut => 2.99,
        ChocolateFlavor::Mint => 3.99,
    }
}

fn main() {
    dbg!(prize(ChocolateFlavor::default()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prize() {
        assert_eq!(prize(ChocolateFlavor::Milk), 2.49);
        assert_eq!(prize(ChocolateFlavor::Dark), 2.49);
        assert_eq!(prize(ChocolateFlavor::White), 2.49);
        assert_eq!(prize(ChocolateFlavor::Hazelnut), 2.99);
        assert_eq!(prize(ChocolateFlavor::Mint), 3.99);
    }
}
