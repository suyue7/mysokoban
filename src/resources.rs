//这个文件主要定义了一些资源和相关的结构体，用于管理游戏的输入、状态、时间和事件。
use crate::audio::AudioStore;
use crate::events::Event;
use ggez::event::KeyCode;
use specs::World;

use std::{fmt::{self, Display}, time::Duration};

// Resources
#[derive(Default)]
pub struct InputQueue {//InputQueue 结构体包含一个 keys_pressed 字段，是一个 Vec<KeyCode>，用于存储按下的键。
    pub keys_pressed: Vec<KeyCode>,
}
//register_resources 函数接受一个 World 的可变引用，并向其中插入一些资源的默认实例
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}

#[derive(PartialEq)]
pub enum GameplayState {//GameplayState 枚举表示游戏的不同状态：Playing：游戏进行中。Won：游戏胜利。
    Playing,
    Won,
    Failed,
}

impl Display for GameplayState {//为 GameplayState 实现 Display 特性，以便能够将其转换为字符串。
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
            GameplayState::Failed => "Failed",
        })?;
        Ok(())
    }
}

impl Default for GameplayState {//为 GameplayState 实现 Default 特性，默认状态为 Playing。
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Default)]
pub struct Gameplay {//Gameplay 结构体包含两个字段：state：游戏状态，类型为 GameplayState。moves_count：移动次数，类型为 u32。
    pub state: GameplayState,
    pub moves_count: usize,
}

#[derive(Default)]
pub struct Time {//Time 结构体包含一个 delta 字段，类型为 Duration，用于表示时间增量。
    pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {//EventQueue 结构体包含一个 events 字段，是一个 Vec<Event>，用于存储事件。
    pub events: Vec<Event>,
}
