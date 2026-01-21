use gpui::{AssetSource, RenderImage, Result, SharedString};
use image::Frame;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::sync::Arc;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "*.png"]
pub struct Assets;

impl Assets {
    /// Get icon image as RenderImage for use with img() element
    pub fn get_icon(name: &str) -> Option<Arc<RenderImage>> {
        let data = Self::get(name)?;
        let img = image::load_from_memory(&data.data).ok()?;
        let mut rgba = img.into_rgba8();

        // Convert RGBA to BGRA as GPUI expects
        for pixel in rgba.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        let frame = Frame::new(rgba);
        Some(Arc::new(RenderImage::new(vec![frame])))
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        match Self::get(path) {
            Some(f) => Ok(Some(f.data)),
            None => Err(anyhow::anyhow!("asset not found: {}", path)),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(|p| SharedString::from(p.to_string()))
            .collect())
    }
}
