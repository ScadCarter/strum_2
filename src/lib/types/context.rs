pub trait Context {
    fn get_terminal(&mut self) -> Option<&mut super::Terminal>;
}
