struct Gfx {}
struct Win {
    closed: bool
}

impl crate::Graphics for Gfx {}
impl crate::Window for Win {
    fn close(&mut self) {
        self.closed = true
    }
    fn closed(&self) -> bool {
        self.closed
    }
}

#[test]
#[should_panic]
fn error_hook() {
    struct App {}
    impl crate::Game<Gfx, Win> for App {
        fn load() -> Self {
            panic!("this should panic")
        }

        fn update(&mut self, _win: &mut Win) {
            todo!()
        }

        fn draw(&self, _gfx: &mut Gfx) {
            todo!()
        }
    }
    crate::run::<App, Gfx, Win, crate::DefaultErrorHook>(Gfx {}, Win { closed:false })
}