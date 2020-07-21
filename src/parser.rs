use std::fmt;
use std::fmt::Write;

use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash, Default)]
pub struct Money(pub i64);

impl Money {
    pub fn abs(self) -> Money {
        Money(self.0.abs())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 0 {
            f.write_char('-')?;
        }
        let mut num = self.0.abs();
        let mut len = 0;
        let mut rev_num = 0;
        while num > 0 {
            rev_num *= 10;
            rev_num += num % 10;
            num /= 10;
            len += 1;
        }
        for i in 0..len {
            if i == len - 2 {
                f.write_char('.')?;
            }
            write!(f, "{}", rev_num % 10)?;
            rev_num /= 10;
        }
        Ok(())
    }
}

struct MoneyVisitor;

impl<'de> Visitor<'de> for MoneyVisitor {
    type Value = Money;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a money value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut input = value.chars();
        let sign = match input.next() {
            Some('+') => 1,
            Some('-') => -1,
            Some(c) => return Err(Error::invalid_value(Unexpected::Char(c), &"a + or -")),
            None => return Err(Error::invalid_length(value.len(), &"a + or -")),
        };
        match input.next() {
            Some(' ') => {}
            Some(c) => return Err(Error::invalid_value(Unexpected::Char(c), &"a space")),
            None => return Err(Error::invalid_length(value.len(), &"a space")),
        };
        match input.next() {
            Some('$') => {}
            Some(c) => return Err(Error::invalid_value(Unexpected::Char(c), &"a dollar sign")),
            None => return Err(Error::invalid_length(value.len(), &"a dollar sign")),
        };
        let mut num = 0;
        for c in input {
            if let Some(d) = c.to_digit(10) {
                num *= 10;
                num += d as i64;
            } else if c != '.' {
                return Err(Error::invalid_value(Unexpected::Char(c), &"a number"));
            }
        }
        Ok(Money(num * sign))
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Money, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MoneyVisitor)
    }
}
