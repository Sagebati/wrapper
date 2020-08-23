#![no_std]

pub trait Wrap<R> {
    fn wrap(self) -> R;
}

///
/// ```rust
/// use wapprer::Wrap;
/// use std::ops::Add;
/// fn foo() -> Result<u32, u32> {
///     fn inner_fn() -> Result<u32,u32> {
///         Ok(2)
///     }
///     let x = inner_fn()?;
///     x.add(5).wrap()
/// }
/// ```
///
impl<T, E> Wrap<Result<T, E>> for T {
    fn wrap(self) -> Result<T, E> {
        Ok(self)
    }
}

///
/// ```rust
/// use wapprer::Wrap;
/// fn foo() -> Option<u32> {
///     let x = 5;
///     match x {
///         0..=2 => 1,
///         3..=5 => 0,
///         _ => 2
///     }.wrap()
/// }
/// ```
///
impl<T> Wrap<Option<T>> for T {
    fn wrap(self) -> Option<T> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Wrap;

    #[test]
    fn option_wrap() {
        let x = 2i32.wrap();
        if let Some(_) = x {
            assert!(true)
        }
        fn inference_test() -> Option<u32> {
            2.wrap()
        }
        assert_eq!(inference_test(), Some(2));
    }

    #[test]
    fn result_wrap() {
        let x: Result<u32, u32> = 4u32.wrap();
        if let Ok(y) = x {
            assert_eq!(y, 4u32)
        }
        fn inference_test() -> Result<&'static str, &'static str> {
            "test".wrap()
        }
        assert_eq!(inference_test(), Ok("test"))
    }
}
