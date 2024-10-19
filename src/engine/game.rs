use crate::engine::renderer::Renderer;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
    fn draw(&mut self, renderer: &Renderer);
}
