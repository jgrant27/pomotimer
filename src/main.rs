extern crate termion;
extern crate pbr;

use self::termion::{color, style};
use self::termion::color::Color;
use self::termion::event::Key;
use self::termion::input::TermRead;

use std::io::{Write, stdout, stdin};
use std::{thread, time};
use self::pbr::ProgressBar;

const ONE_SEC: time::Duration = time::Duration::from_millis(1000);
const TASK_TIME: usize = 25 * 60;
const REST_TIME: usize = 5 * 60;
const LONG_REST_TIME: usize = 15 * 60;

const PB_WIDTH: usize = 80;
const TITLE: &'static str = "POMODORO TIMER";
const POMODORO: &'static str = r#"
                                   `.-:://///:-```
                    `.-://:.--...-+syyyyyyyys/:/+++---`
               `-/osyyyyyyys//+++//:+syyyyy+:+++++:+s+++oooo++/-.
            `:+yyyyyyyyyyyyyys//+++++//+++:/+++++:ossyyyyyyyyyyyyso/.
          `/syyyyso+/////////++:-++++/-///:.++++-+++////////++oyyyyyys:`
         ./++oosyoo++++//////++++++++:/++++:++++/+++++++++++++///+yyyyyy/`
      -osyyyyyyyyyyyyyyyyyyso++/:/+++/++++++++++++/://+++++++++++//syyyyys.
     +yyyyyyyyyyyyyyyyyyyyys+////++++++++++++++++++//syyyyyyyyyyyyyyyyyyyys.
    /yyyyyyyyyyyyyyyyyyyys/:+++++//////+++++///++++++:+yyyyyyyyyyyyyyyyyoo+:
   .yyyyyyyyyyyyyyyyyyyyo:+////++osyyyyyyyyyyyo+//++++//yyyyyyyyyyyyyyyyyyyo.
   /yyyyyyyyyyyyyyyyyyy/-/+oyyyyyyyyyyyyyyyyyyyyyo/:+++:+yyyyyyyyyyyyyyyyyyys`
   oyyyyyyyyyyyyyyyyyssoyyyyyyyyyyyyyyyyyyyyyyyyyyyy+//+:yyyyyyyyyyyyyyyyyyyy:
  `syyyyyyyyyyyyyyyy/+yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyo/-yyyyyyyyyyyyyyyyyyyy+
  `syyyyyyyyyyyyyys-+yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyysoyyyyyyyyyyyyyyyyyyy+
  `syyyyyyyyyyyyyy:+yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy+oyyyyyyyyyyyyyyyyyy+
   oyyyyyyyyyyyyyo/yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyys.yyyyyyyyyyyyyyyyyy/
   .syyyyyyyyyyyyoyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy:oyyyyyyyyyyyyyyyyy.
   /-o:/yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy++yyyyyyyyyyyyyyyy/
   /s:`-ysyyyyyyyyyyyyyyyyyyyyyyyyys/--/ys---+yyyyyyyyyyyyyyyyyyyyyyyyyyyy+--
   `syo::.ysyyyyyyyyyyyyyyyyyyyyyyyyos- o+ -:oyyyyyyyyyyyyyyyyyyyyyyyyys+-:s/
    /yyyo::-+yosyyyyyyyyyyyyyyyyyyyy+.`/sso+`.yyyyyyyyyyyyyyyyyyyyyyy+:-+yyy.
     oyyyyy+::.oys+yy/+yyyyyyyyyyyyo::::o+::/syyyyyyyyyyyyyyyyyyys/::+syyyy+
     .syyyyyyyo/:-.oy..yy+yyyyyyyyyyyyys+syyyyyyyyyyyyyyyyyyso/::/oyyyyyyys`
      -yyyyyyyyyyyo/:-.:/`os+:yy/oyy/yyo +yyyyyyyyyysoo+/////osyyyyyyyyyys.
       -syyyyyyyyyyyyyyso+//::::--::`//:`-///////////+osyyyyyyyyyyyyyyyyy-
        .syyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyys.
         `+yyyyyyyyyyyyyyyyyyyyyyyyyyyyy+syyyyyyyyyyyyyyyyyyyyyyyyyyyyo`
           -syyyyyyyyyyyyyyyyyyyyyyyyy/` `/syyyyyyyyyyyyyyyyyyyyyyyys:
             :syyyyyyyyyyyyyyyyyyyyy+.`````.+yyyyyyyyyyyyyyyyyyyyys/`
              `:oyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyys/`
                 ./syyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyys+-
                    ./oyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyo/.
                        .:/osyyyyyyyyyyyyyyyyyyyyyyys+/:.
                              `.-::///++++////:--.`
"#;


pub fn get_progressbar(width: usize, time: usize) -> ProgressBar<std::io::Stdout> {
    let mut pb = ProgressBar::new(time as u64);
    pb.format(" ░░▌ ");
    pb.show_speed = false;
    pb.show_counter = false;
    pb.show_percent = false;
    pb.set_width(Some(width));
    pb
}

pub fn run_progressbar(mut pb: ProgressBar<std::io::Stdout>) {
    let mut cnt = 0;
    while pb.is_finish == false {
        pb.inc();
        thread::sleep(ONE_SEC);
        cnt = cnt + 1;
        if pb.total == cnt {
            pb.finish();
        }
    }
}

pub fn print_greet_pomo() {
    let greet = format!(" {} - PRESS 'ENTER' KEY TO START", TITLE);
    println!("\n{}{}{}\n{}{}\n{}{}",
             termion::cursor::Hide,
             termion::clear::All,
             termion::cursor::Goto(1, 1),
             color::Fg(color::Green), greet,
             color::Fg(color::Red), POMODORO);
}

pub fn wait_for_key() {
    let stdin = stdin();
    for c in stdin.keys() {
        break;
    }
}

pub fn do_task_and_rest(task: usize) {
    print_timer_pomo(format!("Task {}/4", task));
    print!("{}", color::Fg(color::Green));
    let mut tpb = get_progressbar(PB_WIDTH, TASK_TIME);
    run_progressbar(tpb);
    print!("{}", "\x07".repeat(task));
    if task < 4 {
        print_timer_pomo(format!("Rest {}/4", task));
        print!("{}", color::Fg(color::Blue));
        let mut rpb = get_progressbar(PB_WIDTH, REST_TIME);
        run_progressbar(rpb);
        print!("{}", "\x07".repeat(task));
    }
}

pub fn do_tasks_and_rests() {
    for task in 1..5 {
        do_task_and_rest(task);
    }
}

pub fn do_long_rest() {
    print_timer_pomo(format!("Long Rest"));
    println!("{}", color::Fg(color::Yellow));
    let mut rpb = get_progressbar(PB_WIDTH, LONG_REST_TIME);
    run_progressbar(rpb);
}

pub fn reset_display() {
    println!("{}{}\n",
             termion::cursor::Show,
             color::Fg(color::Reset),);
}

pub fn print_timer_pomo(msg: String) {
    println!("\n{}{}\n{} {}\n {}{}{}{}",
             termion::clear::All,
             termion::cursor::Goto(1, 1),
             color::Fg(color::Green), TITLE,
             color::Fg(color::Reset), msg,
             color::Fg(color::Red), POMODORO);
}

fn main() {
    print_greet_pomo();
    wait_for_key();
    do_tasks_and_rests();
    do_long_rest();
    reset_display();
}
