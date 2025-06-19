use std::{cmp::Ordering, fmt::{Debug, Display}, str::FromStr};
use std::ops::Index;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum VersionField {
    Major,
    Minor,
    Build
}

#[derive(Debug, PartialEq, Clone)]
pub enum VersionParsingError<T> where T: FromStr {
    TooManyParts,
    TooFewParts,
    InvalidPart(T::Err)
}
impl<T> Display for VersionParsingError<T> where T: FromStr, <T as FromStr>::Err: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyParts => write!(f, "too many parts (expects 3)"),
            Self::TooFewParts => write!(f, "too few parts (expects 3)"),
            Self::InvalidPart(x) => write!(f, "a sub-part could not be parsed, with error '{x}'")
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Version<T>  {
    major: T,
    minor: T,
    build: T
}
impl<T> From<Version<T>> for [T; 3] {
    fn from(value: Version<T>) -> Self {
        [value.major, value.minor, value.build]       
    }
}
impl<T> Debug for Version<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ major: {:?}, minor: {:?}, build: {:?} }}", &self.major, &self.minor, &self.build)
    }
}
impl<T> Display for Version<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}
impl<T> PartialOrd for Version<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Version<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        }
        else {
            match self.major.cmp(&other.major) {
                Ordering::Equal => {
                    //The first numbers are equal, so we compare the second numbers
                    match self.minor.cmp(&other.minor) {
                        Ordering::Equal => {
                            //Second numbers are equal, so we compare the revison.
                            self.build.cmp(&other.build)
                        },
                        x => x
                    }
                },
                x => x
            }
        }
    }
}
impl<T> FromStr for Version<T> where T: FromStr {
    type Err = VersionParsingError<T>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').map(|x| x.trim()).collect();
        if parts.len() == 3 {
            Ok (
                Self {
                    major: parts[0].parse().map_err(|x| VersionParsingError::InvalidPart(x))?,
                    minor: parts[1].parse().map_err(|x| VersionParsingError::InvalidPart(x))?,
                    build: parts[2].parse().map_err(|x| VersionParsingError::InvalidPart(x))?
                }
            )
        }
        else if parts.len() < 3 {
            Err(VersionParsingError::TooFewParts)
        }
        else {
            Err(VersionParsingError::TooManyParts)
        }
    }
}
impl<T> Index<VersionField> for Version<T> {
    type Output = T;
    fn index(&self, index: VersionField) -> &Self::Output {
        match index {
            VersionField::Major => &self.major,
            VersionField::Minor => &self.minor,
            VersionField::Build => &self.build
        }
    }
}
impl<T> Version<T> {
    pub const fn new(major: T, minor: T, build: T) -> Self {
        Self {
            major,
            minor,
            build
        }
    }

    pub fn major(&self) -> &T {
        &self.major
    }
    pub fn minor(&self) -> &T {
        &self.minor
    }
    pub fn build(&self) -> &T {
        &self.build
    }
}

#[test]
pub fn test_version_functions() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 3, 4);
    let v3 = Version::new(1, 3, 5);
    let v4 = Version::new(1, 0, 1);
    let v5 = Version::new(0, 1, 4);
    let v6 = Version::new(1, 1, 4);
    let v7 = Version::new(2, 0, 0);

    assert_eq!(format!("{v1}"), "1.0.0");
    assert!(v1 < v2 && v2 < v3, "v1 !< v2 || v2 !<v3");
    assert!(v1 < v4, "v1 !< v4");
    assert!(v5 < v6, "v5 !< v6");

    assert!(v3 > v1, "v3 !> v1");
    assert!(v2 > v1, "v2 !> v1");
    assert!(v3 > v5, "v3 !> v5");
    assert!(v7 > v1, "v7 !> v1");

    assert_eq!(v1, v1, "eq failed!");

    let mut list = vec![v1, v2, v3, v4, v5, v6, v7];
    list.sort();
    assert_eq!(list, vec![v5, v1, v4, v6, v2, v3, v7]);

    let v1_str: String = v1.to_string();
    let v1_decoded: Result<Version<_>, _> = v1_str.parse();
    assert_eq!(v1_decoded.unwrap(), v1);

    let dummy_decoded: Result<Version<i32>, _> = ".4.0".parse();
    assert!(dummy_decoded.is_err());
}