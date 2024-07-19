//这段代码定义了一些函数，用于在游戏世界中创建不同类型的实体。
//每个函数都接收一个 World 对象和一个 Position 对象（以及其他一些参数），然后在世界中创建相应的实体。
//这些实体代表游戏中的墙、地板、箱子、箱子位置和玩家。
use crate::components::*;
use specs::{Builder, World, WorldExt};

//创建墙
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_static("/images/wall.png".to_string()))
        .with(Wall {})
        .with(Immovable)
        .build();
}
//创建地板
pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable::new_static("/images/floor.png".to_string()))
        .build();
}
//创建箱子
pub fn create_box(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            format!("/images/box_{}.png", colour),
            //format!("/images/box_{}_2.png", colour),
        ]))
        .with(GameBox { colour })
        .with(Movable)
        .build();
}
//创建箱子位置
pub fn create_box_spot(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable::new_static(format!(
            "/images/box_spot_{}.png",
            colour
        )))
        .with(BoxSpot { colour })
        .build();
}
//创建玩家
pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            "/images/player_1.png".to_string(),
            "/images/player_2.png".to_string(),
            "/images/player_3.png".to_string(),
        ]))
        .with(Player {})
        .with(Movable)
        .build();
}
