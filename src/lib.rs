use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct SubModule(i32);

impl SubModule {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}

impl Display for SubModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubModule({})", self.0)
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
