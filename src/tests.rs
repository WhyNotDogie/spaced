use crate::error::Result;

struct Man {
    closed: bool
}

impl crate::Manager for Man {}

impl crate::Graphics for Man {
    type Error = anyhow::Error;
}
impl crate::Window for Man {
    fn close(&mut self) {
        self.closed = true
    }
    fn closed(&self) -> bool {
        self.closed
    }
}

#[test]
#[should_panic]
fn error_hook_panic() {
    struct App {}
    impl crate::Game<Man> for App {
        fn load() -> Result<Self> {
            panic!("this should panic")
        }

        fn update(&mut self, _win: &mut Man) -> Result<()> {
            todo!()
        }

        fn draw(&self, _gfx: &mut Man) -> Result<()> {
            todo!()
        }
    }
    let man = Man { closed: false };
    crate::run::<App, crate::DefaultErrorHook, _>(man);
}

#[test]
#[should_panic]
fn error_hook_result() {
    struct App {}
    impl crate::Game<Man> for App {
        fn load() -> Result<Self> {
            Err(anyhow::anyhow!("This should panic too").into())
        }

        fn update(&mut self, _win: &mut Man) -> Result<()> {
            todo!()
        }

        fn draw(&self, _gfx: &mut Man) -> Result<()> {
            todo!()
        }
    }
    let man = Man { closed: false };
    crate::run::<App, crate::DefaultErrorHook, _>(man);
}