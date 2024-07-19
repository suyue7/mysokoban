//这段代码实现了 GameplayStateSystem 系统，用于检查游戏状态（如游戏是否已胜利）。
//它通过遍历所有箱子和箱子位置的匹配情况，决定游戏状态是进行中还是胜利。
use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::{
    components::{GameBox, BoxSpot, Position},
    resources::{Gameplay, GameplayState},
};

pub struct GameplayStateSystem;

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, GameBox>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, positions, boxes, box_spots) = data;

        let boxes_by_position: HashMap<(u8, u8), &GameBox> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();
        if gameplay_state.moves_count>100{
            gameplay_state.state = GameplayState::Failed;
        }
        else{
            for (box_spot, position) in (&box_spots, &positions).join() {
            
                if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                    if the_box.colour == box_spot.colour {
                        continue;
                    } else {
                        gameplay_state.state = GameplayState::Playing;
                        return;
                    }
                } else {
                    gameplay_state.state = GameplayState::Playing;
                    return;
                }
            }
            gameplay_state.state = GameplayState::Won;
        }
        
    }
}
