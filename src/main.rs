use dioxus::prelude::*;
use std::time::Duration;

const STYLE: &str = r#"
* {
    box-sizing: border-box;
    -webkit-tap-highlight-color: transparent;
}

body {
    margin: 0;
    font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #e0e5ec;
    color: #4a5568;
}

.app {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #e0e5ec;
    padding: 20px;
}

.pomodoro {
    width: 380px;
    padding: 45px 35px;
    border-radius: 35px;

    background: #e0e5ec;

    box-shadow:
        15px 15px 30px #bec3c9,
        -15px -15px 30px #ffffff;

    text-align: center;
}

.title {
    margin: 0 0 30px;
    font-size: 28px;
    font-weight: 700;
    letter-spacing: 1px;
    color: #3f4856;
}

.timer {
    width: 230px;
    height: 230px;

    margin: 0 auto 35px;

    display: flex;
    align-items: center;
    justify-content: center;

    border-radius: 50%;

    background: #e0e5ec;

    box-shadow:
        inset 10px 10px 20px #bec3c9,
        inset -10px -10px 20px #ffffff;
}

.time {
    font-size: 48px;
    font-weight: 600;
    letter-spacing: 2px;
    color: #3f4856;
    font-variant-numeric: tabular-nums;
}

.status {
    margin-bottom: 30px;
    font-size: 14px;
    color: #7b8491;
    letter-spacing: 1px;
}

.controls {
    display: flex;
    gap: 18px;
    justify-content: center;
}

.button {
    border: none;
    outline: none;

    padding: 15px 30px;

    border-radius: 15px;

    background: #e0e5ec;

    color: #4a5568;

    font-size: 15px;
    font-weight: 600;

    cursor: pointer;

    box-shadow:
        7px 7px 14px #bec3c9,
        -7px -7px 14px #ffffff;

    transition: all 0.15s ease;
}

.button:hover {
    color: #2f6fed;
}

.button:active {
    box-shadow:
        inset 5px 5px 10px #bec3c9,
        inset -5px -5px 10px #ffffff;

    transform: translateY(1px);
}

.reset {
    padding: 15px 20px;
}
"#;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut seconds = use_signal(|| 25 * 60);
    let mut running = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;

                if running() && seconds() > 0 {
                    seconds -= 1;
                }

                if seconds() == 0 {
                    running.set(false);
                }
            }
        });
    });

    let minutes = seconds() / 60;
    let secs = seconds() % 60;

    rsx! {
        style { {STYLE} }

        div {
            class: "app",

            div {
                class: "pomodoro",

                h1 {
                    class: "title",
                    "Pomodoro"
                }

                div {
                    class: "timer",

                    div {
                        class: "time",
                        "{minutes:02}:{secs:02}"
                    }
                }

                div {
                    class: "status",

                    if running() {
                        "Focus mode"
                    } else if seconds() == 0 {
                        "Session complete"
                    } else {
                        "Ready to focus"
                    }
                }

                div {
                    class: "controls",

                    button {
                        class: "button",

                        onclick: move |_| {
                            running.set(!running());
                        },

                        if running() {
                            "Pause"
                        } else {
                            "Start"
                        }
                    }

                    button {
                        class: "button reset",

                        onclick: move |_| {
                            running.set(false);
                            seconds.set(25 * 60);
                        },

                        "Reset"
                    }
                }
            }
        }
    }
}
