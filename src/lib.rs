use std::{panic, any, process};

pub mod types;
pub mod error;

#[cfg(test)]
mod tests;

pub trait Game<Gfx: Graphics, Win: Window> {
    fn load() -> Self;
    fn update(&mut self, win: &mut Win);
    fn draw(&self, gfx: &mut Gfx);
}

pub trait Graphics: std::panic::UnwindSafe {
    
}

pub trait ErrorHook {
    fn hook(info: Box<dyn any::Any + Send>);
}

pub trait Window: std::panic::UnwindSafe {
    fn close(&mut self);
    fn closed(&self) -> bool;
}

pub fn run<G: Game<Gfx, Win>, Gfx: Graphics, Win: Window, Hook: ErrorHook>(mut gfx: Gfx, mut win: Win) {
    match panic::catch_unwind(move || {
        let mut game = G::load();
        while !win.closed() {
            game.update(&mut win);
            game.draw(&mut gfx);}
    }) {
        Ok(_) => {}
        Err(e) => {
            Hook::hook(e);
            process::exit(1);
        }
    };
}

pub struct DefaultErrorHook {}

impl ErrorHook for DefaultErrorHook {
    fn hook(info: Box<dyn any::Any + Send>) {
        match info.downcast::<&str>() {
            Ok(o) => {
                panic!("{}", o);
            },
            Err(_) => {
                panic!("The program panicked, but the panic message could not be displayed.");
            }
        }
    }
}