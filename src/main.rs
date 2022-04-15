extern crate ncurses;

use std::char;

use ncurses::*;
use raycasting_engine::*;

fn main() {
    initscr();
    noecho();
    raw();

    let mut p = Player::new(Vecf2d::zero(), 0.0);

    // game loop
    loop {
        let ch = getch();

        if ch == 'Q' as i32 {
            break;
        }

        // angle
        if ch == 'a' as i32 {
            p.update_dir(-0.1);
        } else if ch == 'd' as i32 {
            p.update_dir(0.1);
        }

        mvaddstr(
            LINES() - 1,
            0,
            format!(
                "Key pressed: {} | Player: Pos: ({}, {}), Dir: {}",
                char::from_u32(ch as u32).expect("Can not convert char"),
                p.pos().x,
                p.pos().y,
                p.dir()
            ).as_str(),
        );
    }

    getch();
    endwin();
}
