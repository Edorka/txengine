type ClientID = u16;

pub struct Account {
    client: ClientID,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_csv() {
    }
}
