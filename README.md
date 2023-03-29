# bevy_examples

# Event Loops

* Windowed - using [DefaultPlugins](https://docs.rs/bevy/0.10.0/bevy/struct.DefaultPlugins.html) spins up a window and hooks the event loop into the rendering rate for that window (typical game).
* Headless - using [MinimalPlugins](https://docs.rs/bevy/0.10.0/bevy/struct.MinimalPlugins.html) sets up an event loop that runs 'as fast as possible'.

Is there a mode for manually pausing, stepping, advancing the event loop?

See https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.Schedule.html