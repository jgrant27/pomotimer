extern crate termion;
extern crate pbr;

use termion::{color, style};
use termion::color::Color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;

use std::io::{Write, stdout, stdin};
use pbr::ProgressBar;
use std::{thread, time};

const ONE_SEC: time::Duration = time::Duration::from_millis(1000);
const TASK_TIME: u64 = 5;
const REST_TIME: u64 = 2;
const LONG_REST_TIME: u64 = 3;
const PB_WIDTH: usize= 50;


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


fn get_pb_task(width: usize) -> ProgressBar<std::io::Stdout> {
    let mut pb = ProgressBar::new(TASK_TIME);
    pb.format(" ▌▌░ ");
    pb.show_speed = false;
    pb.show_counter = false;
    pb.show_percent = false;
    pb.set_width(Some(width));
    pb
}

fn print_greet_pomo() {
    let (w, h) = terminal_size().unwrap();
    let greet = "          PRESS 'ENTER' KEY TO START THE POMODORO TIMER";
    println!("\n{}{}{}\n{}{}\n{}{}",
             termion::cursor::Hide,
             termion::clear::All,
             termion::cursor::Goto(1, 1),
             color::Fg(color::Green), greet,
             color::Fg(color::Red), POMODORO);
}

fn print_timer_pomo(msg: String) {
    println!("\n{}{}\n{}{}\n{}{}",
             termion::clear::All,
             termion::cursor::Goto(1, 1),
             color::Fg(color::Green), msg,
             color::Fg(color::Red), POMODORO);
}


fn main() {

    print_greet_pomo();

    let stdin = stdin();
    //let stdin = stdin.lock();

    for c in stdin.keys() {
        break;
    }

    for task in 1..5 {
        print!("{}", "\x07".repeat(task));
        print_timer_pomo(format!("          Task {}/4", task));
        print!("{}", color::Fg(color::Green));
        let mut tpb = get_pb_task(50);
        for _ in 1..TASK_TIME {
            tpb.inc();
            thread::sleep(ONE_SEC);
        }
        print_timer_pomo(format!("          Rest {}/4", task));
        print!("{}", color::Fg(color::Blue));
        let mut rpb = get_pb_task(10);
        for _ in 1..REST_TIME {
            rpb.inc();
            thread::sleep(ONE_SEC);
        }
    }

    print_timer_pomo(format!("          Last rest"));
    println!("{}", color::Fg(color::Yellow));
    let mut rpb = get_pb_task(60);
    for _ in 1..LONG_REST_TIME {
        rpb.inc();
        thread::sleep(ONE_SEC);
    }

    println!("{}{}\n",
             termion::cursor::Show,
             color::Fg(color::Reset),);

}