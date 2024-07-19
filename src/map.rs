//从一个字符串表示的地图中加载游戏元素，并将它们添加到游戏的世界中。
use crate::components::{BoxColour, Position};
use crate::entities::*;
use specs::World;
//地图载入
pub const MAPS: [&str; 5] = ["
N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N
N N N N N N N W W W W W W W W
N N N N N N N W W W . . . . W
N N N N N N N W . . . B . . W
N N N N N N N W . . B . . . W
N N N N N N N W . P . . . . W
N N N N N N N W . . . . S . W
N N N N N N N W . . S . . . W
N N N N N N N W . . . . . . W
N N N N N N N W W W W W W W W
",
"
N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N
N N N N N N N W W W W W W W W W W
N N N N N N N W W . . . . . . . W
N N N N N N N W . . B B . P . . W
N N N N N N N W . . B . . . . . W
N N N N N N N W W W W . W W W W W
N N N N N N N W . . . . S . . . W
N N N N N N N W . . S . . W W W W
N N N N N N N W . . . . . . S . W
N N N N N N N W W W W W W W W W W
",
"
N N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N N
N N N N N N W W W W W W W W W W W W
N N N N N N W . . . . S . W . . . W
N N N N N N W . B W . . P W . . . W
N N N N N N W . . W . . . W . . . W
N N N N N N W W W . . . . . . B . W
N N N N N N W . . . . S . W . B . W
N N N N N N W . . S W . W W W . . W
N N N N N N W . . . W . . S . B . W
N N N N N N W W W W W W W W W W W W
",
"
N N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N N
N N N N N N W W W W W W W W W W W W
N N N N N N W . . . . . . W . . . W
N N N N N N W . . . . . . B . . . W
N N N N N N W . B W . . P W . . . W
N N N N N N W . . W . . . W . . . W
N N N N N N W W W . . . . . . B . W
N N N N N N W S . . S S . W . B . W
N N N N N N W . . S W . W W W . . W
N N N N N N W . . . W . . S . B . W
N N N N N N W W W W W W W W W W W W
",
"
N N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N N
N N N N N N N N N N N N N N N N N N
N N N N N N W W W W W W W W W W W W
N N N N N N W . . W . . . W . . . W
N N N N N N W . . W . . . W . B . W
N N N N N N W . B W . . . W . . . W
N N N N N N W . . . . . . W . . . W
N N N N N N W W W . . . . . . B . W
N N N N N N W S . . S S . W . B . W
N N N N N N W . . S W . W W W . . W
N N N N N N W . . . W . . S . B P W
N N N N N N W W W W W W W W W W W W
",
];
pub fn load_map(world: &mut World, map_string: String) {
    // 读取所有行
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => create_floor(world, position),//创建地板
                "W" => {//创建地板和墙
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {//创建地板和玩家
                    create_floor(world, position);
                    create_player(world, position);
                }
                // "BB" => {//创建地板和蓝色箱子
                //     create_floor(world, position);
                //     create_box(world, position, BoxColour::Blue);
                // }
                "B" => {//创建地板和箱子
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Red);
                }
                // "BS" => {//创建地板和蓝色箱子目标点。
                //     create_floor(world, position);
                //     create_box_spot(world, position, BoxColour::Blue);
                // }
                "S" => {//创建地板和箱子目标点。
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Red);
                }
                "N" => {
                    // Ignore 'N' character to avoid creating any object for it
                }
                _ => {
                    // Handle any other unexpected characters
                    println!("Warning: Unrecognized character '{}' in map at position ({}, {})", column, x, y);
                }
            }
        }
    }
}
