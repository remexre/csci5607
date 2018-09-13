/// Provides simple type annotations.
#[macro_export]
macro_rules! the {
    ($ty:ty, $expr:expr) => {{
        let x: $ty = $expr;
        x
    }};
}
