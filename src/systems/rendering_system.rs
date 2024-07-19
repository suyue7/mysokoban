use crate::components::*;
use crate::resources::*;
use crate::constants::TILE_WIDTH;

//use eframe::egui::text_edit;
use ggez::{Context, graphics::{self, DrawParam, Image, Color, spritebatch::SpriteBatch, TextFragment, PxScale}};
use specs::{Join, ReadStorage, System, Read};
use glam::Vec2;
use itertools::Itertools;

use std::{time::Duration, collections::HashMap};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32, font_size: f32, color: Color) {
        let fragment = TextFragment {
            text: text_string.to_string(),
            color: Some(color),
            scale: Some(PxScale::from(font_size)),
            ..Default::default()
        };

        let text = graphics::Text::new(fragment);
        let destination = Vec2::new(x, y);

        graphics::queue_text(self.context, &text, Vec2::new(0.0, 0.0), None);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
        let path_index = match renderable.kind() {
            RenderableKind::Static => 0,
            RenderableKind::Animated => ((delta.as_millis() % 1000) / 250) as usize,
        };
        renderable.path(path_index)
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;

        // graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        for (position, renderable) in rendering_data.iter() {
            let image_path = self.get_image(renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            let z = position.z;
            let draw_param = DrawParam::new().dest(Vec2::new(x, y));
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_param);
        }

        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                let image = Image::new(self.context, image_path).expect("expected image");
                let mut sprite_batch = SpriteBatch::new(image);

                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("expected render");
            }
        }

        let text_color = Color::new(0.0, 0.0, 0.0, 1.0);
        let text_color1=Color::new(1.0,1.0,1.0,1.0);
        let text_color2=Color::new(1.0,1.0,0.0,1.0);
        self.draw_text(&gameplay.state.to_string(), 300.0, 430.0, 28.0, text_color);
        if gameplay.state==GameplayState::Playing{
            self.draw_text("Remaining steps", 300.0, 470.0, 32.0, text_color);
            self.draw_text(&(100 - gameplay.moves_count).to_string(), 300.0, 510.0, 32.0, text_color);
        }
        else if gameplay.state==GameplayState::Failed{
            self.draw_text("You are falied!", 200.0, 470.0, 32.0, text_color1);
            self.draw_text("Press Enter to replay this level, ", 200.0, 510.0, 28.0, text_color1);
            self.draw_text("or press ESC to exit the game. ", 200.0, 540.0, 28.0, text_color1);
        }
        if gameplay.state==GameplayState::Won{
            self.draw_text("Congratulations on the victory!", 150.0, 470.0, 32.0, text_color2);
            self.draw_text("Press Enter to proceed to the next", 150.0, 510.0, 28.0, text_color2);
            self.draw_text("level or press ESC to exit.", 150.0, 540.0, 28.0, text_color2);
        }
        //let fps = format!("FPS: {:.0}", timer::fps(self.context));
        //self.draw_text(&fps, 300.0, 250.0, 32.0, text_color);

        graphics::present(self.context).expect("expected to present");
    }
}
