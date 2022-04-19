extern crate ncurses;

use std::{char, f32::consts::PI, time::Instant};

use ncurses::*;

use raycasting_engine::*;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nodelay(stdscr(), true);
    raw();

    let map = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1,
        1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1,
        1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    init_pair(2, COLOR_RED, COLOR_BLACK);
    init_pair(3, COLOR_BLUE, COLOR_BLUE);

    let map_size = Veci2d::new(10, 18);
    let mut p = Player::new(Vecf2d::new(2.0, 2.0), 0.0);

    let mut maxx = 0;
    let mut maxy = 0;
    getmaxyx(stdscr(), &mut maxy, &mut maxx);

    let mut tp = Instant::now();
    let mut elapsed_time;
    let speed = 15.0;
    // game loop
    loop {
        elapsed_time = tp.elapsed().as_secs_f32();
        tp = Instant::now();

        let ch = getch();

        if ch == 'Q' as i32 {
            break;
        }

        // angle
        if ch == 'a' as i32 {
            p.update_dir(speed * -1.0 * elapsed_time);
        } else if ch == 'd' as i32 {
            p.update_dir(speed * elapsed_time);
        }
        //movement
        else if ch == 'w' as i32 {
            p.move_forward(speed * elapsed_time);
            if map[(p.pos().y as i32 * map_size.x + p.pos().x as i32) as usize] != 0 {
                p.move_backward(speed * elapsed_time);
            }
        } else if ch == 's' as i32 {
            p.move_backward(speed * elapsed_time);
            if map[(p.pos().y.round() as i32 * map_size.x + p.pos().x.round() as i32) as usize] != 0
            {
                p.move_forward(speed * elapsed_time);
            }
        }

        // drawing loop
        let fov = PI / 4.0; // 60deg
        let mrd = 15.0; // maximium render distance

        for x in 0..maxx {
            let ray_angle = (p.dir() - fov / 2.0) + (x as f32 / maxx as f32) * fov;

            // find distance to wall
            let step_size = 0.1;
            let mut distance_to_wall = 0.0;

            let mut hit_wall = false;
            let ray_dir = Vecf2d::new(ray_angle.sin(), ray_angle.cos());

            while !hit_wall && distance_to_wall < mrd {
                distance_to_wall += step_size;
                let test_pos = Veci2d::new(
                    (p.pos().x + ray_dir.x * distance_to_wall) as i32,
                    (p.pos().y + ray_dir.y * distance_to_wall) as i32,
                );

                if test_pos.x < 0
                    || test_pos.x >= map_size.x
                    || test_pos.y < 0
                    || test_pos.y >= map_size.y
                {
                    hit_wall = true;
                    distance_to_wall = mrd;
                } else {
                    if map[(test_pos.y * map_size.x + test_pos.x) as usize] == 1 {
                        hit_wall = true;
                    }
                }
            }

            let ceiling = ((maxy as f32 / 2.0) - (maxy as f32 / distance_to_wall)) as i32;
            let floor = maxy - ceiling;

            // let shade_scale = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
            let shade_scale = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];

            for y in 0..maxy {
                if y <= ceiling {
                    attron(COLOR_PAIR(3));
                    mvaddch(y, x, ' ' as chtype);
                    attroff(COLOR_PAIR(3));
                } else if y > ceiling && y <= floor {
                    attron(COLOR_PAIR(2));
                    mvaddch(
                        y,
                        x,
                        shade_scale[(distance_to_wall / mrd * (shade_scale.len() - 2) as f32)
                            .round() as usize] as chtype,
                    );
                    attroff(COLOR_PAIR(2));
                } else {
                    // mvaddch(y, x, '$' as chtype);
                    let b = 1.0 - (y as f32 - maxy as f32 / 2.0) / (maxy as f32 / 2.0);
                    let b_shade: chtype;
                    if b < 0.25 {
                        b_shade = '^' as chtype;
                    } else if b < 0.5 {
                        b_shade = '`' as chtype;
                    } else if b < 0.75 {
                        b_shade = '\'' as chtype;
                    } else if b < 0.9 {
                        b_shade = '.' as chtype;
                    } else {
                        b_shade = ' ' as chtype;
                    }
                    attron(COLOR_PAIR(1));
                    mvaddch(y, x, b_shade);
                    attroff(COLOR_PAIR(1));
                }
            }
        }

        let pressed_k = match char::from_u32(ch as u32) {
            Some(ch) => ch,
            None => '-',
        };

        mvaddstr(
            LINES() - 1,
            0,
            format!(
                "Key pressed: {} | Player: Pos: ({}, {}), Dir: {} - ({}, {}) | FPS: {}",
                pressed_k,
                p.pos().x,
                p.pos().y,
                p.dir(),
                p.dir_as_yx().1,
                p.dir_as_yx().0,
                1.0 / elapsed_time
            )
            .as_str(),
        );
        draw_map(&map, map_size, p.pos());
    }

    endwin();
}

fn draw_map(map: &[i32], map_size: Veci2d, player_pos: Vecf2d) {
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            match map[(y * map_size.x + x) as usize] {
                1 => {
                    mvaddch(y, x, 'I' as chtype);
                }
                0 => {
                    mvaddch(y, x, 'o' as chtype);
                }
                _ => {}
            }
        }
    }

    mvaddch(
        player_pos.y.round() as i32,
        player_pos.x.round() as i32,
        'P' as chtype,
    );
}
