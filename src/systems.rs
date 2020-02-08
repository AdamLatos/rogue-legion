use super::{Map, Monster, Name, Player, Position, TileType, Velocity, Viewshed};
use legion::filter::filter_fns::{changed, component};
use legion::query::{IntoQuery, Read, Write};
use legion::schedule::Schedulable;
use legion::system::SystemBuilder;
use rltk::{console, field_of_view, Point};
use std::cmp::{max, min};

pub fn build_visibility_system() -> Box<dyn Schedulable> {
    SystemBuilder::<()>::new("VisibilitySystem")
        .with_query(<(Write<Viewshed>, Read<Position>)>::query().filter(changed::<Position>()))
        .with_query(<Read<Viewshed>>::query().filter(component::<Player>() & changed::<Position>()))
        .write_resource::<Map>()
        .build(move |_, world, map, (query, query_player)| {
            for (mut viewshed, pos) in query.iter(&mut *world) {
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &**map);
                viewshed
                    .visible_tiles
                    .retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1);
            }

            // If this is the player, reveal what they can see
            for viewshed in query_player.iter(&mut *world) {
                for t in map.visible_tiles.iter_mut() {
                    *t = false
                }
                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        })
}

pub fn build_move_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("MoveSystem")
        .with_query(<(Write<Position>, Read<Velocity>)>::query())
        .read_resource::<Map>()
        .write_resource::<Point>()
        .build(move |_, world, (map, ppos), query| {
            for (mut pos, vel) in query.iter(&mut *world) {
                let destination_idx = map.xy_idx(pos.x + vel.x, pos.y + vel.y);
                if map.tiles[destination_idx] != TileType::Wall {
                    pos.x = min(map.width - 1, max(0, pos.x + vel.x));
                    pos.y = min(map.height - 1, max(0, pos.y + vel.y));
                }
                ppos.x = pos.x;
                ppos.y = pos.y;
            }
        })
}

pub fn build_ai_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("AiSystem")
        .with_query(<(Read<Viewshed>, Read<Name>)>::query().filter(component::<Monster>()))
        .read_resource::<Point>()
        .build(move |_, world, ppos, query| {
            for (viewshed, name) in query.iter(&mut *world) {
                if viewshed.visible_tiles.contains(ppos) {
                    console::log(&format!("{} shouts insults", name.name));
                }
            }
        })
}
