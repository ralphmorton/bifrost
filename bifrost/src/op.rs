pub trait Op {
    type Output;

    fn id() -> &'static str;

    #[cfg(feature = "remote")]
    fn execute(&self) -> Self::Output;
}
