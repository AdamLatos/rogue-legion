use super::{Player, State, Velocity};
use legion::query::{IntoQuery, Read, Write};
use legion::world::World;
use rltk::{Rltk, VirtualKeyCode};

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let query = <(Read<Player>, Write<Velocity>)>::query();
    //let map = world.resources.get::<Vec<TileType>>().expect("lol");
    for (_, mut vel) in query.iter(world) {
        vel.x = delta_x;
        vel.y = delta_y;
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => try_move_player(0, 0, &mut gs.world),
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.world),
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.world),
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.world),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.world),
            VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.world),
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.world),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.world),
            VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.world),
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.world),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.world),
            VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.world),
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.world),
            VirtualKeyCode::Q => panic!("Quit"),
            _ => {}
        },
    };
}
