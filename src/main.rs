use dioxus::prelude::*;
use std::time::Duration;

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
            }
        });
    });

    let minutes = seconds() / 60;
    let secs = seconds() % 60;

    rsx! {
        div {
            style: "text-align:center; padding:40px;",

            h1 { "Pomodoro" }

            h2 {
                "{minutes:02}:{secs:02}"
            }

            button {
                onclick: move |_| running.set(!running()),
                if running() { "Pause" } else { "Start" }
            }

            button {
                onclick: move |_| {
                    running.set(false);
                    seconds.set(25 * 60);
                },
                "Reset"
            }
        }
    }
}
