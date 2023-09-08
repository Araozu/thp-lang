/// Represents a qualified datatype of the compiler.
///
/// A datatype is composed of a path, e.g. `base.Str`, `base.Num`
#[derive(PartialEq)]
pub struct Datatype {
    t: String,
}

impl Datatype {
    pub fn new(t: String) -> Datatype {
        Datatype { t }
    }

    pub fn str() -> Datatype {
        Datatype {
            t: String::from("base.Str"),
        }
    }

    pub fn num() -> Datatype {
        Datatype {
            t: String::from("base.Num"),
        }
    }

    pub fn bool() -> Datatype {
        Datatype {
            t: String::from("base.Bool"),
        }
    }

    pub fn clone(&self) -> Datatype {
        Datatype { t: self.t.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::Datatype;

    #[test]
    fn should_create_datatype() {
        let str = Datatype::new(String::from("base.Str"));
        assert_eq!("base.Str", str.t);
    }

    #[test]
    fn should_create_primitive_datatypes() {
        assert_eq!("base.Str", Datatype::str().t);
        assert_eq!("base.Num", Datatype::num().t);
        assert_eq!("base.Bool", Datatype::bool().t);
    }

    #[test]
    fn should_compare() {
        let s1 = Datatype::str();
        let s2 = Datatype::str();

        assert_eq!(true, (s1 == s2));
    }
}
