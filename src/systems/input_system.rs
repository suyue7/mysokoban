// //这段代码实现了 InputSystem 系统，用于处理玩家的输入并更新游戏状态。
// //它通过解析玩家按下的键来移动玩家及其推的箱子，并在必要时触发相应的事件。
use crate::components::*;
use crate::constants::*;
use crate::events::{EntityMoved, Event};
use crate::resources::{EventQueue, Gameplay, InputQueue};
use crate::GameplayState;
use ggez::event::KeyCode;
use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use std::collections::HashMap;
//InputSystem 结构体是空的，因为所有的逻辑都在 System 实现中。
pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // SystemData 定义了系统运行时所需的所有数据。
    // 包括事件队列、输入队列、游戏状态、实体、位置存储、玩家组件、可移动组件和不可移动组件。
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, InputQueue>,
        Write<'a, Gameplay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    // 解构传入的系统数据
    fn run(&mut self, data: Self::SystemData) {
        let (
            mut events,
            mut input_queue,
            mut gameplay,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;
        
        // 初始化一个空的向量，用于存储需要移动的实体。
        let mut to_move = Vec::new();

        // 创建可移动和不可移动实体的位置和 ID 映射
        // 遍历所有玩家实体的位置，并获取输入队列中的第一个按键
        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                let mut can_move = true;
                // 根据按键确定移动的方向和范围
                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    if let Some(id) = mov.get(&pos) {
                        to_move.push((key, id.clone()));
                    } else if immov.get(&pos).is_some() {
                        // 找到不可移动物体，停止并清除to_move清单
                        to_move.clear();
                        events.events.push(Event::PlayerHitObstacle {});
                        // 增加不可移动设置
                        can_move = false;
                        break;
                    } else {
                        break;
                    }
                }
                // 可移动才增加移动计数。
                if can_move && !to_move.is_empty() {
                    gameplay.moves_count += 1;
                }
            }
        }
        if gameplay.state == GameplayState::Failed || gameplay.state == GameplayState::Won {
            // 如果游戏状态是 Failed 或 Won，则忽略输入
            return;
        }
        //遍历范围内的位置，检查是否存在可移动或不可移动的实体
        //如果找到可移动实体，将其添加到待移动列表。如果找到不可移动实体，停止并清除待移动列表，同时触发障碍事件。如果找到空位置，则停止。
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
            //根据待移动列表中的信息，实际移动实体的位置，并触发移动事件
            events.events.push(Event::EntityMoved(EntityMoved { id }));
        }
    }
}
