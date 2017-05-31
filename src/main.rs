extern crate ncurses;
extern crate rand;

use std::char;
use ncurses::*;
use rand::Rng;

fn random_char_num() -> i32 {
    rand::thread_rng().gen_range(33, 127)
}

fn new_line() -> Vec<i32> {
    let mut line = Vec::new();
    for _ in 0..60 {
        line.push(random_char_num());
    }
    line
}

fn print_line<'a>(characters: &'a Vec<i32>) {
    let mut my_line = String::new();
    for i in characters {
        my_line.push(char::from_u32(*i as u32).unwrap());
    }
    my_line.push('\n');
    printw(&my_line);
}

fn print_char(ch: i32) {
    printw(format!("{}", char::from_u32(ch as u32).expect("Invalid char")).as_ref());
}

fn exit() {
    endwin();
    println!("exiting!");
}

fn main()
{
    /* Setup ncurses. */
    let screen = initscr();
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_RED);

    // /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    printw("Type along with me!\n");

    let mut line_number = 0;

    'lines: loop {
        line_number += 1;

        let current_line = new_line();
        print_line(&current_line);
        let mut position = 0;

        'characters: loop {
            let ch = getch();
            attroff(COLOR_PAIR(1));
            let current_char = current_line[position as usize];

            if ch == 0004
            {
                break 'lines;
                exit();
            } else if ch == 13 || ch == 10
            {
                printw("\n");
                break 'characters;
            } else if ch == KEY_BACKSPACE
            {
                position -= 1;
                wmove(screen, line_number + 1, position);
                delch();
            } else if ch != current_char
            {
                position += 1;
                attron(COLOR_PAIR(1));
                print_char(ch);
            } else
            {
                position += 1;
                print_char(ch);
            }

            refresh();
        }
    }
    exit();
}
