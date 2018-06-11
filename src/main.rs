#[macro_use]
extern crate gfx;
extern crate ggez;
extern crate rand;
extern crate rayon;

mod implementation;
use implementation::Viewport;

use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

pub struct MainState {
    frame: u32,
    port: Viewport,
    shader: graphics::Shader<implementation::AdditionalData>,
    zoom_position: Option<graphics::Point2>,
    zooming_in: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_background_color(ctx, graphics::BLACK);
        let port = Viewport::from_xywh(0.0, 0.0, 6.4, 3.6);
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
        _yrel: i32
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
    let ctx = &mut ContextBuilder::new("Mandelbrot Set", "SuperCuber")
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::True)
                .dimensions(
                    implementation::SCREEN_RESOLUTION.0 as u32,
                    implementation::SCREEN_RESOLUTION.1 as u32,
                ),
        )
        .build()
        .expect("window context");

    let state = &mut MainState::new(ctx).expect("main state");

    event::run(ctx, state).expect("run game");
}
