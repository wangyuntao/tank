use self::map::Map;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::ContextBuilder;

mod map;

pub fn start() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("map_editor", "wangyuntao")
        .window_setup(WindowSetup::default().title("Map Editor"))
        .window_mode(WindowMode::default().dimensions(1200.0, 630.0))
        .add_resource_path("./assets/resources")
        .build()
        .unwrap();

    let mut map = Map::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut map) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
