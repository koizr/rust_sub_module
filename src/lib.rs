#[derive(Debug, PartialEq)]
pub struct SubModule(i32);

impl SubModule {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(SubModule(42), SubModule::new(42));
    }
}
