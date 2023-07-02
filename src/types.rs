use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn red(&self) -> u8 {
        self.r
    }
    pub fn green(&self) -> u8 {
        self.g
    }
    pub fn blue(&self) -> u8 {
        self.b
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b
        }
    }
}

/// A handle to an image. Under the hood, this is just an `Arc<[u8]>` (a reference), so you don't have to worry about passing references to it everywhere and you can just run .clone().
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image {
    handle: Arc<[u8]>,
    pub encoding: ImageEncoding
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Self {
            handle: Arc::clone(&self.handle),
            encoding: self.encoding.clone()
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ImageEncoding {
    Jpg,
    #[default]
    Png
}

impl Image {
    pub fn raw_data(&self) -> &[u8] {
        &self.handle
    }
    pub fn encoding(&self) -> ImageEncoding {
        self.encoding
    }
}