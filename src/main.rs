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
    let mut ctx = Box::new(Context::new(Some(lib::create_terminal().unwrap())));

    loop {
        let mut c = &mut ctx;

        lib::draw::<Context>(&mut c, vec![]);

        lib::step(None, |event| match event {
            crossterm::event::Event::Key(_event) => {}
            crossterm::event::Event::Mouse(_) => {}
            crossterm::event::Event::Resize(width, height) => {
                println!("New size {}x{}", width, height);
            }
        });
    }
}
