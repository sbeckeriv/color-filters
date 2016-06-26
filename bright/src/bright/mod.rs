use image;
use image::GenericImage;
use std::fs::File;
use std::path::Path;

pub trait Bright: Send + Sync{
    fn lighten(&self, red: u8, blue: u8, green: u8) -> (u8,u8,u8);
}
pub use self::standard::Standard;
pub use self::channel::Channel;
pub use self::channel_zero::ChannelZero;
pub use self::duotone::DuoTone;
mod duotone;
mod channel_zero;
mod channel;
mod standard;
