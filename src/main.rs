// Rust sokoban
// main.rs
use ggez::{
    conf, event::{self, KeyCode, KeyMods, MouseButton}, graphics::{self, DrawParam, Image}, timer, Context, GameResult
};
//use imgui::*;
use specs::{RunNow, World, WorldExt, Join};
use std::path;
//use tokio::task;
mod audio; // 声音
mod components; // 定义游戏的组件
mod constants; // 定义游戏中的常量
mod entities; // 定义游戏中的实体
mod events; // 处理事件
mod map; // 处理地图
mod resources; // 定义资源
mod systems; // 定义系统
use crate::map::MAPS;
use crate::audio::*;
use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

struct MainMenu {
    background_image: Option<Image>,
}

impl event::EventHandler<ggez::GameError> for MainMenu {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::Color::BLACK);
        if let Some(background_image) = &self.background_image {
            graphics::draw(context, background_image, DrawParam::default())?;
        }

        // 绘制关卡按钮
        for i in 0..5 {
            let level_button_rect = graphics::Rect::new(100.0 + i as f32 * 120.0, 100.0, 100.0, 50.0);
            let level_button = graphics::Mesh::new_rectangle(
                context,
                graphics::DrawMode::fill(),
                level_button_rect,
                graphics::Color::WHITE,
            )?;
            graphics::draw(context, &level_button, DrawParam::default())?;

            let level_button_text = graphics::Text::new(format!("Level {}", i + 1));
            graphics::draw(
                context,
                &level_button_text,
                (ggez::mint::Point2 { x: 120.0 + i as f32 * 120.0, y: 115.0 }, graphics::Color::BLACK),
            )?;
        }

        // 绘制“开始游戏”按钮
        let start_button_rect = graphics::Rect::new(300.0, 200.0, 200.0, 50.0);
        let start_button = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            start_button_rect,
            graphics::Color::WHITE,
        )?;
        graphics::draw(context, &start_button, DrawParam::default())?;

        let start_button_text = graphics::Text::new("Start Game");
        graphics::draw(
            context,
            &start_button_text,
            (ggez::mint::Point2 { x: 350.0, y: 215.0 }, graphics::Color::BLACK),
        )?;

        // 绘制“退出”按钮
        let quit_button_rect = graphics::Rect::new(300.0, 300.0, 200.0, 50.0);
        let quit_button = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            quit_button_rect,
            graphics::Color::WHITE,
        )?;
        graphics::draw(context, &quit_button, DrawParam::default())?;

        let quit_button_text = graphics::Text::new("Quit");
        graphics::draw(
            context,
            &quit_button_text,
            (ggez::mint::Point2 { x: 375.0, y: 315.0 }, graphics::Color::BLACK),
        )?;

        graphics::present(context)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, context: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                // 点击关卡按钮
                for i in 0..5 {
                    if x >= 100.0 + i as f32 * 120.0 && x <= 200.0 + i as f32 * 120.0 && y >= 100.0 && y <= 150.0 {
                        event::quit(context);
                        if let Err(e) = start_game(i) {
                            println!("Error starting game: {}", e);
                        }
                        return;
                    }
                }
                // 点击进入游戏按钮
                if x >= 300.0 && x <= 500.0 && y >= 200.0 && y <= 250.0 {
                    //event::quit(context);
                    if let Err(e) = start_game(0) {
                        println!("Error starting game: {}", e);
                    }
                }
                // 点击退出按钮
                else if x >= 300.0 && x <= 500.0 && y >= 300.0 && y <= 350.0 {
                    event::quit(context);
                }
            }
            _ => {}
        }
    }
}

struct Game {
    world: World, //Game 结构体包含一个 World 对象，World 是 specs 的核心，包含所有实体和组件。
    background_image: Option<Image>,
    current_level: usize,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // 运行输入系统
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // 运行游戏状态系统
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // 获取并更新时间资源
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context);
        }
        // 运行事件系统
        {
            let mut es = EventSystem { context };
            es.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        if let Some(background_image) = &self.background_image {
            graphics::draw(context, background_image, DrawParam::default())?;
        }
        
        // 渲染游戏实体
        {   
            let mut rs = RenderingSystem {
                context,
            };
            rs.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(//处理按键事件，将案件添加到输入队列中
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        match keycode {
            KeyCode::Return => {
                // 处理“Enter”键事件：进入下一关
                if self.world.read_resource::<Gameplay>().state == GameplayState::Won {
                    self.current_level += 1;
                    if self.current_level < MAPS.len() {
                        initialize_level(&mut self.world, self.current_level as i32);
                        initialize_sounds(&mut self.world, _context);
                    } else {
                        println!("所有关卡完成！");
                        event::quit(_context);
                    }
                }
                if self.world.read_resource::<Gameplay>().state == GameplayState::Failed{
                    if self.current_level < MAPS.len() {
                        initialize_level(&mut self.world, self.current_level as i32);
                        initialize_sounds(&mut self.world, _context);
                    } else {
                        println!("所有关卡完成！");
                    }
                }
            }
            KeyCode::Escape => {
                // 处理“Esc”键事件：退出游戏
                event::quit(_context);
            }
            _ => {
                let mut input_queue = self.world.write_resource::<InputQueue>();
                input_queue.keys_pressed.push(keycode);
            }
        }
    }
}

pub fn initialize_level(world: &mut World, index: i32) {
    // 从世界资源中读取音频存储
    //let audio_store = world.read_resource::<AudioStore>().clone();

    // 使用新的作用域，在此作用域内删除所有实体，确保不会同时持有不可变和可变引用
    {
        delete_all_entities(world);
        register_components(world);
        register_resources(world);
    }

    // 最后将音频资源插入世界并加载地图
    //world.insert(audio_store);
    load_map(world, MAPS[index as usize].to_string());
}

fn delete_all_entities(world: &mut World) {
    let entities: Vec<_> = world.entities().join().collect();
    for entity in entities {
        world.delete_entity(entity).expect("Failed to delete entity");
    }
}

fn start_game(level: usize) -> GameResult {
    let mut world = World::new();
    // register_components(&mut world);
    // register_resources(&mut world);
    // 初始化音频资源
    //world.insert(AudioStore::default());
    initialize_level(&mut world, level as i32);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(780.0, 600.0).resizable(true))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;
    initialize_sounds(&mut world, &mut context);

    let background_image = Image::new(&mut context, "/images/background.png")?;
    
    let game = Game {
        world,
        background_image: Some(background_image),
        current_level: level,
    };

    event::run(context, event_loop, game)
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban - Main Menu"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;
    let background_image = Image::new(&mut context, "/images/main_menu_background.png")?;

    let main_menu = MainMenu {
        background_image: Some(background_image),
    };

    event::run(context, event_loop, main_menu)
}
