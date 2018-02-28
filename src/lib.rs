//! https://ru.wikipedia.org/wiki/Сирена_(сеть)
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate encoding_rs;

use std::fmt;
use std::str;
use std::str::FromStr;
use std::borrow::Cow;

use encoding_rs::KOI8_R;

macro_rules! gen_display {
    ($t: ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    }
}

macro_rules! gen_as {
    () => {
        pub fn as_str(&self) -> Cow<str> {
            let (s, _, _) = KOI8_R.decode(&self.0);
            s
        }

        pub fn as_bytes(&self) -> &[u8] {
            &self.0
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone, Copy)]
pub struct AircraftCode([u8; 3]);

gen_display!(AircraftCode);
impl AircraftCode {
    gen_as!();
}

#[derive(Debug)]
pub enum AircraftCodeParseError {
    InvalidLength(usize),
    InvalidLetter(char),
}

impl fmt::Display for AircraftCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AircraftCodeParseError::InvalidLength(len) => write!(f, "invalid length {}, expected 3", len),
            AircraftCodeParseError::InvalidLetter(c) => write!(f, "invalid character {}, expected [А-Я]", c),
        }
    }
}

impl std::error::Error for AircraftCodeParseError {
    fn description(&self) -> &str {
        "aircraft code parse error"
    }
}

impl FromStr for AircraftCode {
    type Err = AircraftCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.chars().count() != 3 {
            return Err(AircraftCodeParseError::InvalidLength(value.len()));
        }
        for c in value.chars() {
            if c.is_ascii_digit() || (c >= 'А' && c <= 'Я') {
                continue;
            } else {
                return Err(AircraftCodeParseError::InvalidLetter(c));
            }
        }
        let (koi8str, _, _) = KOI8_R.encode(value);
        let mut bytes = [0; 3];
        bytes.copy_from_slice(&koi8str);
        Ok(AircraftCode(bytes))
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone, Copy)]
pub struct AirlineCode([u8; 2]);

gen_display!(AirlineCode);

impl AirlineCode {
    gen_as!();

    /// Reconstruct AirlineCode from AirlineCode.as_bytes()
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let mut mine = [0; 2];

        mine.copy_from_slice(bytes);
        AirlineCode(mine)
    }
}

#[derive(Debug)]
pub enum AirlineCodeParseError {
    InvalidLength(usize),
    InvalidLetter(char),
    TooManyDigits(u32),
}


impl fmt::Display for AirlineCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AirlineCodeParseError::InvalidLength(len) => write!(f, "invalid length {}, expected 2", len),
            AirlineCodeParseError::InvalidLetter(c) => write!(f, "invalid character {}, expected [А-Я]", c),
            AirlineCodeParseError::TooManyDigits(digits) => write!(f, "got {} digits, only 1 allowed", digits),
        }
    }
}

impl std::error::Error for AirlineCodeParseError {
    fn description(&self) -> &str {
        "airline code parse error"
    }
}

impl FromStr for AirlineCode {
    type Err = AirlineCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.chars().count() != 2 {
            return Err(AirlineCodeParseError::InvalidLength(value.len()));
        }
        let mut digits = 0;
        for c in value.chars() {
            if c >= 'А' && c <= 'Я' {
                continue;
            } else if c.is_ascii_digit() {
                digits += 1;
                continue;
            } else {
                return Err(AirlineCodeParseError::InvalidLetter(c));
            }
        }
        // can't be 2 digits,
        // https://ru.wikipedia.org/wiki/Код_авиакомпании_ИАТА#Внутренняя_система_кодирования_в_бывшем_СССР
        if digits > 1 {
            return Err(AirlineCodeParseError::TooManyDigits(digits));
        }
        let (koi8str, _, _) = KOI8_R.encode(value);
        let mut bytes = [0; 2];
        bytes.copy_from_slice(&koi8str);
        Ok(AirlineCode(bytes))
    }
}

/// 3 letter airport code
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone, Copy)]
pub struct AirportCode([u8; 3]);

gen_display!(AirportCode);

impl AirportCode {
    gen_as!();

    /// Reconstruct AirportCode from AirportCode.as_bytes()
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let mut mine = [0; 3];

        mine.copy_from_slice(bytes);
        AirportCode(mine)
    }
}

#[derive(Debug)]
pub enum AirportCodeParseError {
    InvalidLength(usize),
    InvalidLetter(char),
}

impl fmt::Display for AirportCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AirportCodeParseError::InvalidLength(len) => write!(f, "invalid length {}, expected 3", len),
            AirportCodeParseError::InvalidLetter(c) => write!(f, "invalid character {}, expected [А-Я]", c),
        }
    }
}

impl std::error::Error for AirportCodeParseError {
    fn description(&self) -> &str {
        "airport code parse error"
    }
}

impl FromStr for AirportCode {
    type Err = AirportCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.chars().count() != 3 {
            return Err(AirportCodeParseError::InvalidLength(value.len()));
        }
        for c in value.chars() {
            if c >= 'А' && c <= 'Я' {
                continue;
            } else {
                return Err(AirportCodeParseError::InvalidLetter(c));
            }
        }
        let (koi8str, _, _) = KOI8_R.encode(value);
        let mut bytes = [0; 3];
        bytes.copy_from_slice(&koi8str);
        Ok(AirportCode(bytes))
    }
}

/// 3 letter airport code
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone, Copy)]
pub struct CityCode([u8; 3]);

gen_display!(CityCode);

impl CityCode {
    gen_as!();

    /// Reconstruct CityCode from CityCode.as_bytes()
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let mut mine = [0; 3];

        mine.copy_from_slice(bytes);
        CityCode(mine)
    }
}

#[derive(Debug)]
pub enum CityCodeParseError {
    InvalidLength(usize),
    InvalidLetter(char),
}

impl fmt::Display for CityCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CityCodeParseError::InvalidLength(len) => write!(f, "invalid length {}, expected 3", len),
            CityCodeParseError::InvalidLetter(c) => write!(f, "invalid character {}, expected [А-Я]", c),
        }
    }
}

impl std::error::Error for CityCodeParseError {
    fn description(&self) -> &str {
        "city code parse error"
    }
}

impl FromStr for CityCode {
    type Err = CityCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.chars().count() != 3 {
            return Err(CityCodeParseError::InvalidLength(value.len()));
        }
        for c in value.chars() {
            if c >= 'А' && c <= 'Я' {
                continue;
            } else {
                return Err(CityCodeParseError::InvalidLetter(c));
            }
        }
        let (koi8str, _, _) = KOI8_R.encode(value);
        let mut bytes = [0; 3];
        bytes.copy_from_slice(&koi8str);
        Ok(CityCode(bytes))
    }
}

#[test]
fn test_encode_aircraft() {
    let a = "ПУ1";
    let code = AircraftCode::from_str(a).unwrap();
    println!("{:?}", code);
    assert_eq!(a, &format!("{}", code));
    assert_eq!(a, &code.as_str());
}