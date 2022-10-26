pub trait Unit: Clone + PartialEq {
    fn symbol(&self) -> String;
}
