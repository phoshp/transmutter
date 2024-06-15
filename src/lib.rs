pub trait Transmutter<U> {
    type Target;
}

impl<T: ?Sized, U> Transmutter<U> for T {
    type Target = U;
}

fn transmutter<T: ?Sized, U>(source: <T as Transmutter<U>>::Target) -> U {
    source
}

pub fn transmutte<T, U>(value: T) -> U {
    transmutter::<dyn Transmutter<U, Target = T>, U>(value)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    pub struct Dang {
        v1: u32,
        v2: &'static str
    }
    pub struct Bang {
        v1: &'static str,
        v2: &'static str,
    }

    #[test]
    fn it_works() {
        let a = Dang {
            v1: 1,
            v2: "safe transmute"
        };
        let b: Bang = transmutte(a);
        assert_eq!(b.v1, "safe transmute");
    }
}
