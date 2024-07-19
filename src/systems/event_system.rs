//这段代码实现了 EventSystem 系统。
//用于处理不同的游戏事件，如玩家撞到障碍、实体移动和箱子放置在特定位置。
//它还会根据事件播放相应的音效。
use crate::{
    audio::AudioStore,
    components::*,
    events::{BoxPlacedOnSpot, EntityMoved, Event},
    resources::EventQueue,
};
use specs::{Entities, Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct EventSystem<'a> {
    pub context: &'a mut ggez::Context,
}

// System implementation
impl<'a> System<'a> for EventSystem<'a> {
    // 数据
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
        Entities<'a>,
        ReadStorage<'a, GameBox>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, mut audio_store, entities, boxes, box_spots, positions) = data;//解构传入的系统数据

        let mut new_events = Vec::new();// 创建一个存储新事件的向量

        for event in event_queue.events.drain(..) {//遍历事件队列并处理每个事件
            println!("New event: {:?}", event);

            match event {
                Event::PlayerHitObstacle => {//对于 Event::PlayerHitObstacle 事件，播放 "wall" 音效。
                    // play sound here
                    audio_store.play_sound(self.context, &"wall".to_string());
                }
                Event::EntityMoved(EntityMoved { id }) => {//对于 Event::EntityMoved 事件，检查移动的实体是否为箱子，并判断箱子是否被放置在正确的位置上。如果是，触发 BoxPlacedOnSpot 事件。
                    // An entity was just moved, check if it was a box and fire
                    // more events if it's been moved on a spot.
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spots_with_positions: HashMap<(u8, u8), &BoxSpot> =
                            (&box_spots, &positions)
                                .join()
                                .map(|t| ((t.1.x, t.1.y), t.0))
                                .collect::<HashMap<_, _>>();

                        if let Some(box_position) = positions.get(entities.entity(id)) {
                            // Check if there is a spot on this position, and if there
                            // is if it's the correct or incorrect type
                            if let Some(box_spot) =
                                box_spots_with_positions.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot:(box_spot.colour == the_box.colour),
                                }));
                            }
                        }
                    }
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {//对于 Event::BoxPlacedOnSpot 事件，根据是否放置在正确位置播放对应的音效。
                    // play sound here
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };

                    audio_store.play_sound(self.context, &sound.to_string())
                }
            }
        }

        event_queue.events.append(&mut new_events);//将新事件追加到事件队列中
    }
}
