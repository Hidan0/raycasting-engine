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

    dda(Point { x: 0, y: 0 }, Point { x: 0, y: 10 });
    dda(Point { x: 6, y: 5 }, Point { x: 9, y: 10 });
    dda(Point { x: 10, y: 10 }, Point { x: 3, y: 18 });

    getch();
    endwin();
}

fn dda(p1: Point, p2: Point) {
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
        mvaddch(y.round() as i32, x.round() as i32, '#' as u32);
        x += x_inc;
        y += y_inc;
    }
    refresh();
}
