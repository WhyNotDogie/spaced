pub mod error;

pub struct Man {
    win: Win
}

pub struct Win {
    closed: bool
}

pub struct Gfx {

}

impl spaced::Manager for Man {}
impl spaced::Window for Man {
    type Error = error::Error;
    fn close(&mut self) -> Result<(), Self::Error> {
        self.win.closed = true;
        Ok(())
    }
    fn closed(&self) -> bool {
        self.win.closed
    }
}
impl spaced::Graphics for Man {
    type Error = error::Error;
}