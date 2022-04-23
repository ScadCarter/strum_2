pub mod types;

use tui::{backend::CrosstermBackend, Terminal};

pub fn draw<C: types::Context>(state: &mut C, mut components: Vec<&mut dyn types::Component<C>>) {
    for component in components.iter_mut() {
        component.render(state);
    }
}

pub fn create_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, std::io::Error> {
    // setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();

    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

pub fn step<F: Fn(crossterm::event::Event)>(sleep_time: Option<u64>, callback: F) {
    let event = match sleep_time {
        Some(time) => {
            if crossterm::event::poll(std::time::Duration::from_millis(time))
                .unwrap_or_else(|_| false)
            {
                Some(crossterm::event::read().unwrap())
            } else {
                None
            }
        }
        None => match crossterm::event::read() {
            Ok(event) => Some(event),
            // TODO: todo!("figure out how to handle this")
            Err(_) => None,
        },
    };

    match event {
        None => {}
        Some(event) => callback(event),
    }
}

#[cfg(test)]
mod tests {
    mod draw {
        use crate::lib::{draw, types};

        #[derive(Default)]
        struct TestContext {
            pub count: u32,
            terminal: Option<types::Terminal>,
        }

        impl types::Context for TestContext {
            fn get_terminal(&mut self) -> Option<&mut types::Terminal> {
                self.terminal.as_mut()
            }
        }

        impl TestContext {
            pub fn inc_count(&mut self) {
                self.count += 1;
            }
        }

        #[derive(Default)]
        struct TestComponent;
        impl types::Component<TestContext> for TestComponent {
            fn render(&mut self, ctx: &mut TestContext) {
                ctx.inc_count();
            }
        }

        #[test]
        fn should_call_render_on_every_component() {
            let mut first = TestComponent::default();
            let mut second = TestComponent::default();

            let components: Vec<&mut dyn types::Component<TestContext>> =
                vec![&mut first, &mut second];

            let mut ctx = TestContext::default();
            draw(&mut ctx, components);

            assert_eq!(2, ctx.count);
        }
    }

    mod terminal {
        // TODO: figure out how to test this
    }

    mod step {
        // TODO: figure out how to test this
    }
}
