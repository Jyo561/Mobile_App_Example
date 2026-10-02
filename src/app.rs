use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::session::SessionType;

#[derive(Clone, Copy, PartialEq)]
struct Pomodoro {
    session: SessionType,
    remaining: u64,
    running: bool,
    completed: u32,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            session: SessionType::Pomodoro,
            remaining: SessionType::Pomodoro.duration(),
            running: false,
            completed: 0,
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let pomodoro = use_state(Pomodoro::default);

    // Timer
    {
        let pomodoro = pomodoro.clone();

        use_effect_with(pomodoro.running, move |running| {
            let interval = if *running {
                let pomodoro = pomodoro.clone();

                Some(Interval::new(1000, move || {
                    let mut state = *pomodoro;

                    if state.remaining > 0 {
                        state.remaining -= 1;
                    }

                    if state.remaining == 0 {
                        state.running = false;

                        if state.session == SessionType::Pomodoro {
                            state.completed += 1;
                        }
                    }

                    pomodoro.set(state);
                }))
            } else {
                None
            };

            move || {
                drop(interval);
            }
        });
    }
    // Start / Pause
    let start_pause = {
        let pomodoro = pomodoro.clone();

        Callback::from(move |_| {
            let mut state = *pomodoro;

            state.running = !state.running;

            pomodoro.set(state);
        })
    };

    // Reset
    let reset = {
        let pomodoro = pomodoro.clone();

        Callback::from(move |_| {
            let mut state = *pomodoro;

            state.remaining = state.session.duration();
            state.running = false;

            pomodoro.set(state);
        })
    };

    // Skip
    let skip = {
        let pomodoro = pomodoro.clone();

        Callback::from(move |_| {
            let mut state = *pomodoro;

            state.session = state.session.next();
            state.remaining = state.session.duration();
            state.running = false;

            pomodoro.set(state);
        })
    };

    let minutes = pomodoro.remaining / 60;
    let seconds = pomodoro.remaining % 60;

    html! {
        <main>
            <h1>{"Pomodoro"}</h1>

            <h2>
                {pomodoro.session.name()}
            </h2>

            <div>
                <span>
                    {format!("{:02}:{:02}", minutes, seconds)}
                </span>
            </div>

            <div>
                <button onclick={start_pause}>
                    {
                        if pomodoro.running {
                            "Pause"
                        } else {
                            "Start"
                        }
                    }
                </button>

                <button onclick={reset}>
                    {"Reset"}
                </button>

                <button onclick={skip}>
                    {"Skip"}
                </button>
            </div>

            <p>
                {format!(
                    "Completed Pomodoros: {}",
                    pomodoro.completed
                )}
            </p>
        </main>
    }
}
