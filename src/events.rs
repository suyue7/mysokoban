//这个文件定义了一些事件相关的结构体和枚举，用于表示游戏中的各种事件。
pub type EntityId = u32;

#[derive(Debug)]
pub struct EntityMoved {
    pub id: EntityId,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,//一个布尔值，表示箱子是否被放置在正确的位置。
}

#[derive(Debug)]
pub enum Event {
    // 当玩家碰到障碍物（如墙壁）时触发的事件
    PlayerHitObstacle,

    // 当实体移动时触发的事件，包含一个 EntityMoved 结构体实例。
    EntityMoved(EntityMoved),

    // 当箱子被放置在某个位置时触发的事件，包含一个 BoxPlacedOnSpot 结构体实例。
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}
