use std::{panic::{self, UnwindSafe}, process, any::Any};
use error::*;
use serde::{Serialize, Deserialize};

pub use anyhow as __anyhow;

pub mod types;
pub mod error;

#[cfg(test)]
mod tests;

pub trait Manager: UnwindSafe + Graphics + Window {}

pub trait Game<Man: Manager> where Self: Sized {
    fn load() -> Result<Self>;
    fn update(&mut self, io: &mut Man) -> Result<()>;
    fn draw(&self, io: &mut Man) -> Result<()>;
}

pub trait Graphics: UnwindSafe + Sized {
    type Error: Into<anyhow::Error>;
    fn draw<T: Draw + Any>(&mut self, drawable: T) -> Self::Error {
        drawable.draw(self)
    }
}

pub trait Draw {
    fn draw<Gfx: Graphics>(&self, gfx: &mut Gfx) -> Gfx::Error;
}

pub trait ErrorHook {
    fn hook(err: Error, panic: bool);
}

pub trait Window: UnwindSafe {
    type Error: Into<anyhow::Error>;
    fn close(&mut self) -> std::result::Result<(), Self::Error>;
    fn closed(&self) -> bool;
}

pub fn run<G: Game<Man>, Hook: ErrorHook, Man: Manager>(mut man: Man) {
    match panic::catch_unwind(move || {
        let mut game = G::load()?;
        while !man.closed() {
            game.update(&mut man)?;
            game.draw(&mut man)?;
        }
        error::Result::Ok(())
    }) {
        Ok(v) => {
            match v {
                Ok(_) => {}
                Err(e) => {
                    Hook::hook(e, false);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            Hook::hook(anyhow::anyhow!(*e.downcast::<&str>().unwrap_or(Box::new("<>"))).into(), true);
            process::exit(101);
        }
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultErrorHook {}

impl ErrorHook for DefaultErrorHook {
    fn hook(err: Error, panic: bool) {
        if panic {
            panic!("A panic occured: {}", err)
        } else {
            panic!("Error: {}", err)
        }
    }
}

pub mod prelude {
    pub use crate::{
        Game,
        DefaultErrorHook,
        error::{Error, Result},
        types::Color,
        run
    };
}