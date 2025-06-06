use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Local};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};

#[cfg(feature = "notify")]
use notify_rust::Notification;

use crate::common::sleep;
use crate::format::{dur, time};
use crate::print::Printer;

struct TimerState {
    time: Duration,
    duration: Duration,
    increment: Duration,
    cancel: bool,
    paused: bool,
    printer: Printer,
}

fn read_keys(state: Arc<Mutex<TimerState>>) {
    loop {
        let t = state.lock().unwrap();
        if t.cancel {
            break;
        }
        drop(t);
        let key = read().unwrap();
        let mut state = state.lock().unwrap();
        if state.cancel {
            break;
        }
        match key {
            Event::Key(KeyEvent {
                code: KeyCode::Char('p') | KeyCode::Char(' '),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if !state.paused {
                    let left = &(state.time - state.duration);
                    state.printer.erase(format!("{} PAUSED", dur::time(left)));
                }

                state.paused = !state.paused;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let left = &(state.time - state.duration);
                let now_time = time::time(&Local::now());
                state.printer.print(format!(
                    "\x07{now_time}: Timer cancelled (time left: {left})"
                ));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Timer cancelled")
                        .body(&format!("Timer for {dur} cancelled\n (time left: {left})"))
                        .show()
                    {
                        eprintln!("Failed to send notification: {e}");
                    }
                }

                state.cancel = true;
                return;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right | KeyCode::Char('a'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                state.time = state.time - (state.increment * 5);
                let paused = state.paused;
                let left = &(state.time - state.duration);
                state.printer.erase(format!(
                    "{} {}",
                    dur::time(left),
                    if paused { "PAUSED" } else { "" }
                ));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left | KeyCode::Char('d'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                state.time = state.time + (state.increment * 5);
                let paused = state.paused;
                let left = &(state.time - state.duration);
                state.printer.erase(format!(
                    "{} {}",
                    dur::time(left),
                    if paused { "PAUSED" } else { "" }
                ));
            }
            _ => (),
        }
    }
}

pub fn timer(duration: Duration) {
    let now_time = time::time(&Local::now());
    let dur = dur::time(&duration);
    println!("{now_time}: Started timer for {dur}");

    let state = Arc::new(Mutex::new(TimerState {
        time: Duration::zero(),
        duration,
        increment: Duration::seconds(1),
        paused: false,
        cancel: false,
        printer: Printer::new(),
    }));

    let read_state = state.clone();
    std::thread::spawn(move || read_keys(read_state));

    loop {
        sleep(1.0);
        let mut state = state.lock().unwrap();
        if state.cancel {
            drop(state);
            return;
        }

        if !state.paused {
            state.time = state.time + state.increment;
            let left = &(state.time - state.duration);
            state.printer.erase(format!("{}", dur::time(left)));

            if state.time >= state.duration {
                let now_time = time::time(&Local::now());
                state
                    .printer
                    .print(format!("\x07{now_time}: Completed timer for {dur}"));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Timer complete")
                        .body(&format!(
                            "{now_time}: Timer for {dur} complete\nFinished at {}",
                            time::time(&Local::now())
                        ))
                        .show()
                    {
                        eprintln!("Failed to send notification: {e}");
                    }
                }

                state.cancel = true;
                break;
            }
        }
    }
}

pub fn alarm(stop: DateTime<Local>) {
    let now = Local::now();
    print!(
        "{}: Alarm set at {}",
        now.time().format("%H:%M:%S"),
        stop.time()
    );
    if Local::now().date_naive() < stop.date_naive() {
        println!(" (tomorrow)");
    } else {
        println!();
    }
    let mut printer = Printer::new();

    let mut time = Local::now();
    let mut elapsed = Duration::zero();

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        if poll(std::time::Duration::ZERO).unwrap() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) = read().unwrap()
            {
                let left = dur::time(&(stop - time));
                let time = time::time(&stop);
                printer.print(format!(
                    "\x07Alarm for {time} cancelled (time left: {left})",
                ));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Alarm cancelled")
                        .body(&format!("Alarm for {time} cancelled\nTime left: {left}"))
                        .show()
                    {
                        eprintln!("Failed to send notification: {e}");
                    }
                }

                break;
            }
        }

        printer.erase(format!("{}", dur::time(&(stop - time))));
        sleep(1.0);

        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }

        if time >= stop {
            elapsed = Duration::zero();
            time = Local::now();

            if time >= stop {
                let time = time::time(&stop);
                printer.print(format!(
                    "\x07{time}: Alarm complete! (total time {})",
                    dur::time(&(stop - now))
                ));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Alarm complete")
                        .body(&format!(
                            "{time}: Alarm complete! (total time {})",
                            dur::time(&(stop - now))
                        ))
                        .show()
                    {
                        eprintln!("Failed to show notification: {e}");
                    }
                }

                break;
            }
        }
    }
}
