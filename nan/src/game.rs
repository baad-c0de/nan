use mage_core::{image::Point, App, Colour};

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for Game {
    fn tick(&mut self, _tick_input: mage_core::TickInput) -> mage_core::TickResult {
        mage_core::TickResult::Continue
    }

    fn present(&mut self, mut present_input: mage_core::PresentInput) -> mage_core::PresentResult {
        let mut image = present_input.new_image();
        image.draw_string(
            Point::new(1, 1),
            "Hello, World!",
            Colour::Yellow.into(),
            Colour::Black.into(),
        );
        present_input.blit(
            present_input.rect(),
            image.rect(),
            &image,
            Colour::Black.into(),
        );
        mage_core::PresentResult::Changed
    }
}
