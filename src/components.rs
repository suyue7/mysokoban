//这段代码定义了一些游戏组件以及它们的行为，并提供了一个函数来注册这些组件。
//specs 库被用于管理这些组件。组件是游戏中实体的属性，决定了实体的特性和行为。
use specs::{Component, NullStorage, VecStorage, World, WorldExt};

use std::fmt::{self, Display};

// Components
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {//Position 组件用于表示实体在游戏世界中的位置。
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
//Renderable 组件表示实体的渲染信息。
pub enum RenderableKind {//RenderableKind 枚举表示渲染类型（静态或动画）。
    Static,
    Animated,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {//paths：存储渲染图片路径。
    paths: Vec<String>,
}

impl Renderable {//new_static 和 new_animated：创建静态或动画渲染。
    pub fn new_static(path: String) -> Self {
        Self { paths: vec![path] }
    }

    pub fn new_animated(paths: Vec<String>) -> Self {
        Self { paths }
    }

    pub fn kind(&self) -> RenderableKind {//kind：返回渲染类型。
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> String {//path：根据索引返回渲染路径。
        // If we get asked for a path that is larger than the
        // number of paths we actually have, we simply mod the index
        // with the length to get an index that is in range.
        self.paths[path_index % self.paths.len()].clone()
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}//墙

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}//玩家

#[derive(PartialEq)]
pub enum BoxColour {//箱子的颜色
    Red,
    //Blue,
}

impl Display for BoxColour {//用于格式化 BoxColour。
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColour::Red => "red",
            //BoxColour::Blue => "blue",
        })?;
        Ok(())
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct GameBox{
    pub colour: BoxColour,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot{
    pub colour: BoxColour,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;//表示实体是否可移动，使用 NullStorage 存储，因为它们不包含数据。

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;//表示实体是否可移动，使用 NullStorage 存储，因为它们不包含数据。
//register_components 函数在 World 中注册所有定义的组件，使它们可以被使用。
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<GameBox>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}
