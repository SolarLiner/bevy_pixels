use std::time::Duration;

use bevy_app::*;
use bevy_ecs::prelude::*;
use bevy_internal::{
    core::{Time, Timer},
    DefaultPlugins,
};
use bevy_pixels::{PixelsOptions, PixelsPlugin, PixelsResource, PixelsStage};
use bevy_window::WindowDescriptor;
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bevy Pixels CI".to_string(),
            width: 100.,
            height: 100.,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(PixelsOptions {
            width: 100,
            height: 100,
        })
        .insert_resource(Timer::new(Duration::new(1, 0), false))
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_system_to_stage(PixelsStage::Draw, draw.system())
        .add_system_to_stage(CoreStage::Last, quit_on_timer.system())
        .run();
}

fn draw(mut pxbuf: ResMut<PixelsResource>) {
    let frame = pxbuf.pixels.get_frame();
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

fn quit_on_timer(
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if timer.tick(time.delta()).just_finished() {
        app_exit_events.send(AppExit);
    }
}
