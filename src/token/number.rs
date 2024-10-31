use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Number {
    value: f64,
}

impl Number {
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl FromStr for Number {
    type Err = ParseFloatError;
    fn from_str(num_string: &str) -> Result<Self, Self::Err> {
        match num_string.parse::<f64>() {
            Ok(value) => Ok(Number { value }),
            Err(e) => Err(e),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_nan() || other.value.is_nan() {
            false
        } else {
            self.value == other.value
        }
    }
}

impl Eq for Number {}
impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value == self.value.trunc() {
            write!(f, "{:.1}", self.value) // this is to pass codecrafter's test
        } else {
            write!(f, "{}", self.value)
        }
    }
}
