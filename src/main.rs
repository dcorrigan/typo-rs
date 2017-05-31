extern crate ncurses;
extern crate rand;

use std::char;
use ncurses::*;
use rand::Rng;
use std::time::Instant;

// std::time::Instant for measuring

// This returns a number matching a UTF-8 keyboard character
fn random_char_num() -> i32 {
    rand::thread_rng().gen_range(33, 127)
}

fn should_be_a_space() -> bool {
    let num = rand::thread_rng().gen_range(1, 8);
    if num == 7 {
        return true;
    } else {
        return false;
    }
}

fn new_line() -> Vec<i32> {
    let mut line = Vec::new();
    for x in 0..60 {
        if should_be_a_space() && x != 59 {
            line.push(32);
        } else {
            line.push(random_char_num());
        }
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

fn line_errors(typed_line: Vec<i32>, current_line: Vec<i32>) -> i32 {
    let mut err_count = 0;
    for (i, ch) in typed_line.iter().enumerate() {
        if *ch != current_line[i] {
            err_count += 1;
        }
    }
    err_count
}

fn exit(measured_time: f64, errors: i32, wpm: i32) {
    endwin();
    println!("exiting!");
    println!("time: {} minutes", measured_time);
    println!("errors: {}", errors);
    println!("words per minutes: {}", wpm);
}

fn calc_wpm(typed: i32, errors: i32, minutes: f64) -> i32 {
    let total_correct: f64 = (typed as f64 / 5.0) - errors as f64;
    (total_correct / minutes).round() as i32
}

fn main()
{
    /* Setup ncurses. */
    let screen = initscr();
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_RED);

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    printw("Type along with me! Press CTRL+d (EOT) to exit.\n");
    let now = Instant::now();

    let mut line_number = 0;
    let mut total_typed = 0;
    let mut errors = 0;

    'lines: loop {
        line_number += 2;

        let current_line = new_line();
        let mut typed_line = Vec::new();
        print_line(&current_line);
        let mut position = 0;

        'characters: loop {
            let ch = getch();
            attroff(COLOR_PAIR(1));

            if ch == 0004
            {
                errors += line_errors(typed_line, current_line);
                break 'lines;
            } else if ch == 13 || ch == 10
            {
                printw("\n");
                break 'characters;
            } else if position as usize == current_line.len()
            {
                printw("\n");
                break 'characters;
            } else if ch == KEY_BACKSPACE
            {
                position -= 1;
                total_typed -= 1;
                wmove(screen, line_number, position);
                typed_line.pop();
                delch();
            } else if ch != current_line[position as usize]
            {
                position += 1;
                total_typed += 1;
                attron(COLOR_PAIR(1));
                typed_line.push(ch);
                print_char(ch);
            } else
            {
                position += 1;
                total_typed += 1;
                typed_line.push(ch);
                print_char(ch);
            }

            refresh();
        }

        errors += line_errors(typed_line, current_line);
    }

    let elapsed: f64 = now.elapsed().as_secs() as f64 / 60.0;

    exit(elapsed, errors, calc_wpm(total_typed, errors, elapsed));
}
