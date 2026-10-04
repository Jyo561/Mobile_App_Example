pub fn run() -> iced::Result {
    iced::application("Pomodoro", update, view)
        .subscription(subscription)
        .run()
}
