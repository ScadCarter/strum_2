pub trait Component<C> {
    fn render(&mut self, ctx: &mut C);
}
