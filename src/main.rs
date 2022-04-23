mod lib;

struct Context {
    terminal: Option<lib::types::Terminal>,
}

impl Context {
    fn new(terminal: Option<lib::types::Terminal>) -> Self {
        Self { terminal }
    }
}

impl lib::types::Context for Context {
    fn get_terminal(&mut self) -> Option<&mut lib::types::Terminal> {
        self.terminal.as_mut()
    }
}

fn main() {
    let mut ctx = Context::new(Some(lib::create_terminal().unwrap()));

    lib::draw::<Context>(&mut ctx, vec![]);
}
