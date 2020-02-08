rltk::add_wasm_support!();
use legion::query::{IntoQuery, Read};
use legion::schedule::Schedule;
use legion::world::{Universe, World};
use rltk::{Console, GameState, Point, Rltk, RGB};
use std::iter;

mod player;
use player::*;
mod map;
use map::*;
mod components;
use components::*;
mod rect;
use rect::*;
mod systems;
use systems::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub world: World,
    pub scheduler: Schedule,
    pub runstate: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        // handle key inputs
        if self.runstate == RunState::Running {
            // run systems
            self.scheduler.execute(&mut self.world);
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

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
            Name {
                name: "player".to_string(),
            },
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
    let mut rng = rltk::RandomNumberGenerator::new();
    world.insert(
        (),
        map.rooms.iter().skip(1).enumerate().map(|(i, room)| {
            let roll = rng.roll_dice(1, 2);
            let glyph: u8;
            let name: String;
            match roll {
                1 => {
                    glyph = rltk::to_cp437('g');
                    name = "goblin".to_string();
                }
                _ => {
                    glyph = rltk::to_cp437('o');
                    name = "orc".to_string();
                }
            }
            (
                Monster {},
                Name {
                    name: format!("{} #{}", &name, i),
                },
                Position {
                    x: room.center().0,
                    y: room.center().1,
                },
                Renderable {
                    glyph: glyph,
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
    world.resources.insert(Point::new(player_x, player_y));
    let move_system = build_move_system();
    let vis_system = build_visibility_system();
    let ai_system = build_ai_system();

    let scheduler = Schedule::builder()
        .add_system(vis_system)
        .add_system(move_system)
        .add_system(ai_system)
        .flush()
        .build();

    let gs = State {
        world: world,
        scheduler: scheduler,
        runstate: RunState::Running,
    };

    rltk::main_loop(context, gs);
}
