extern crate ncurses;

use ncurses::*;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    initscr();
    noecho();

    start_color();
    
    init_pair(1, COLOR_RED, COLOR_RED);
    init_pair(2, COLOR_BLUE, COLOR_BLUE);
    init_pair(3, COLOR_GREEN, COLOR_GREEN);

    dda(Point { x: 0, y: 0 }, Point { x: 20, y: 50 }, 1);
    dda(Point { x: 0, y: 0 }, Point { x: 50, y: 50 }, 2);
    dda(Point { x: 0, y: 0 }, Point { x: 50, y: 20 }, 3);

    getch();
    endwin();
}

fn dda(p1: Point, p2: Point, color: i16) {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    let step: i32;
    if dx.abs() > dy.abs() {
        step = dx.abs();
    } else {
        step = dy.abs();
    }

    let x_inc: f64 = dx as f64 / step as f64;
    let y_inc: f64 = dy as f64 / step as f64;

    let (mut x, mut y) = (p1.x as f64, p1.y as f64);
    for _ in 1..=step {
        attron(COLOR_PAIR(color));
        mvaddch(y.round() as i32, x.round() as i32, '#' as u32);
        attroff(COLOR_PAIR(color));
        x += x_inc;
        y += y_inc;
    }
    refresh();
}
