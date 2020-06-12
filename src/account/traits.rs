pub trait AccountAble {
    fn balance(&self) -> i32;

    fn name(&self) -> &str;

    fn id(&self) -> i32;
}
