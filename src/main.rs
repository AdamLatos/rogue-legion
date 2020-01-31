rltk::add_wasm_support!();
use legion::query::{IntoQuery, Read, Write};
use legion::schedule::Schedule;
use legion::system::SystemBuilder;
use legion::world::{Universe, World};
use rltk::{Console, GameState, Rltk, RGB};
use std::cmp::{max, min};
use std::iter;

mod player;
use player::*;
mod map;
use map::*;
mod components;
use components::*;
mod rect;
use rect::*;
mod visibility_system;
use visibility_system::*;

pub struct State {
    pub world: World,
    pub scheduler: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        // handle key inputs
        player_input(self, ctx);

        // run systems
        self.scheduler.execute(&mut self.world);

        // render stuff

        // map first
        draw_map(&self.world, ctx);
        // entities second
        let query = <(Read<Position>, Read<Renderable>)>::query();
        for (pos, render) in query.iter_immutable(&self.world) {
            let map = self.world.resources.get::<Map>().expect("no map");
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() {
    let universe = Universe::new();
    let context = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");
    let mut world = universe.create_world();

    let map = Map::new_map_rooms();
    let (player_x, player_y) = map.rooms[0].center();

    world.insert(
        (),
        iter::once((
            Player {},
            Position {
                x: player_x,
                y: player_y,
            },
            Velocity { x: 0, y: 0 },
            Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
            },
            Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            },
        )),
    );
    world.insert(
        (),
        map.rooms.iter().skip(1).map(|room| {
            (
                Position {
                    x: room.center().0,
                    y: room.center().1,
                },
                Renderable {
                    glyph: rltk::to_cp437('g'),
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                },
                Viewshed {
                    visible_tiles: Vec::new(),
                    range: 8,
                },
            )
        }),
    );

    world.resources.insert(map);
    let move_objects = SystemBuilder::new("MoveObjects")
        .with_query(<(Write<Position>, Read<Velocity>)>::query())
        .read_resource::<Map>()
        .build(move |_, world, map, query| {
            for (mut pos, vel) in query.iter(&mut *world) {
                let destination_idx = map.xy_idx(pos.x + vel.x, pos.y + vel.y);
                if map.tiles[destination_idx] != TileType::Wall {
                    pos.x = min(map.width - 1, max(0, pos.x + vel.x));
                    pos.y = min(map.height - 1, max(0, pos.y + vel.y));
                }
            }
        });

    let vis_system = build_visibility_system();

    let scheduler = Schedule::builder()
        .add_system(move_objects)
        .add_system(vis_system)
        .flush()
        .build();

    let gs = State {
        world: world,
        scheduler: scheduler,
    };

    rltk::main_loop(context, gs);
}
