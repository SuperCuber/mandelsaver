#[macro_use]
extern crate gfx;
extern crate ggez;
extern crate rand;
extern crate rayon;

mod implementation;
use implementation::Viewport;

use ggez::{conf, event, graphics, mouse, Context, ContextBuilder, GameResult};

pub struct MainState {
    frame: u32,
    port: Viewport,
    shader: graphics::Shader<implementation::AdditionalData>,
    zoom_position: Option<graphics::Point2>,
    zooming_in: bool,
}

fn get_resolution() -> Result<(u32, u32), std::io::Error> {
    let exe_location = std::env::current_exe()?;
    let config_file = exe_location
        .parent()
        .expect("folder containing exe")
        .join("mandelbrot.conf");
    let contents = std::fs::read_to_string(config_file)?;

    let by_whitespace = contents.split_whitespace().collect::<Vec<_>>();
    let width = by_whitespace[0].parse().expect("width as number");
    let height = by_whitespace[1].parse().expect("height as number");

    Ok((width, height))
}

impl MainState {
    fn new(ctx: &mut Context, resolution: (u32, u32)) -> GameResult<MainState> {
        mouse::set_grabbed(ctx, true);
        graphics::set_background_color(ctx, graphics::WHITE);
        mouse::set_relative_mode(ctx, true);
        let port = Viewport::from_xywh(0.0, 0.0, 6.4, 3.6, resolution);
        Ok(MainState {
            frame: 0,
            shader: graphics::Shader::from_u8(
                ctx,
                include_bytes!("shader/myshader_150.glslv"),
                include_bytes!("shader/myshader_150.glslf"),
                port.to_data(),
                "AdditionalData",
                None,
            )?,
            port: port,
            zoom_position: None,
            zooming_in: true,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        implementation::update(self, ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        implementation::draw(self, ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: event::MouseButton,
        _x: i32,
        _y: i32,
    ) {
        implementation::interrupt(self, ctx)
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        _state: event::MouseState,
        _x: i32,
        _y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        implementation::interrupt(self, ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        _keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool,
    ) {
        implementation::interrupt(self, ctx)
    }
}

fn main() {
    let dimensions = get_resolution().unwrap_or((1920, 1080));
    let ctx = &mut ContextBuilder::new("Mandelbrot Set", "SuperCuber")
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::True)
                .dimensions(dimensions.0, dimensions.1),
        )
        .build()
        .expect("window context");

    let state = &mut MainState::new(ctx, dimensions).expect("main state");

    event::run(ctx, state).expect("run game");
}
