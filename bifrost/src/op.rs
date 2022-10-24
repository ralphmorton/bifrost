pub trait Op {
    type Output;

    fn id() -> &'static str;

    #[cfg(any(feature = "remote", feature = "debug"))]
    fn execute(&self) -> Self::Output;
}
