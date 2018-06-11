use rand::{thread_rng, Rng};

use MainState;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Image, Point2};
use ggez::conf::NumSamples;

use rayon::prelude::*;

pub(crate) const SCREEN_RESOLUTION: (usize, usize) = (1920, 1080);

#[derive(Debug)]
pub(crate) struct Viewport {
    center: Point2,
    size: Point2,
    cache: Option<Image>,
}

gfx_defines!{
    constant AdditionalData {
        center: [f32; 2] = "center",
        size: [f32; 2] = "size",
        resolution: [f32; 2] = "resolution",
        iter: u32 = "iter",
    }
}

impl Viewport {
    pub(crate) fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Viewport {
        Viewport {
            center: Point2::new(x, y),
            size: Point2::new(w, h),
            cache: None,
        }
    }

    fn screen_to_port(&self, point: Point2) -> Point2 {
        let real =
            self.center.x - self.size.x / 2.0 + point.x * self.size.x / SCREEN_RESOLUTION.0 as f32;
        let imag =
            self.center.y - self.size.y / 2.0 + point.y * self.size.y / SCREEN_RESOLUTION.1 as f32;
        let point = Point2::new(real, imag);
        point
    }

    fn render(
        &mut self,
        shader: &graphics::Shader<AdditionalData>,
        ctx: &mut Context,
    ) -> GameResult<()> {
        let canvas = graphics::Canvas::new(
            ctx,
            SCREEN_RESOLUTION.0 as u32,
            SCREEN_RESOLUTION.1 as u32,
            NumSamples::One,
        )?;
        graphics::set_canvas(ctx, Some(&canvas));

        {
            let _lock = graphics::use_shader(ctx, shader);
            shader.send(ctx, self.to_data())?;
            graphics::rectangle(
                ctx,
                graphics::DrawMode::Fill,
                graphics::Rect {
                    x: 0.0,
                    y: 0.0,
                    w: SCREEN_RESOLUTION.0 as f32,
                    h: SCREEN_RESOLUTION.1 as f32,
                },
            )?;
        }

        graphics::set_canvas(ctx, None);

        self.cache = Some(canvas.into_inner());
        Ok(())
    }

    fn draw(
        &mut self,
        shader: &graphics::Shader<AdditionalData>,
        ctx: &mut Context,
    ) -> GameResult<()> {
        if self.cache.is_none() {
            self.render(shader, ctx)?;
        }

        graphics::draw(
            ctx,
            self.cache.as_ref().expect("cache exists after render"),
            Point2::new(0.0, 0.0),
            0.0,
        )
    }

    fn zoom_towards(&mut self, point: Point2) {
        const RATIO: f32 = 0.97;
        self.center = Point2::new(
            self.center.x * RATIO + point.x * (1.0 - RATIO),
            self.center.y * RATIO + point.y * (1.0 - RATIO),
        );
        self.size *= 0.99;
        self.cache = None;
    }

    fn zoom_out(&mut self) {
        // self.center *= 0.999;
        self.size *= 1.02;
        self.cache = None;
    }

    pub(crate) fn to_data(&self) -> AdditionalData {
        let iter = 256.0;// * (1.0 / self.size.x);
        AdditionalData {
            center: [self.center.x, self.center.y],
            size: [self.size.x, self.size.y],
            iter: iter as u32,
            resolution: [SCREEN_RESOLUTION.0 as f32, SCREEN_RESOLUTION.1 as f32],
        }
    }
}

fn select_zoom_position_screen(
    ctx: &mut Context,
    screenshot: &Image,
) -> GameResult<Option<Point2>> {
    let width = screenshot.get_dimensions().w as usize;
    let height = screenshot.get_dimensions().h as usize;

    let color_converted = screenshot
        .to_rgba8(ctx)?
        .chunks(4)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|chunk| (chunk[0], chunk[1], chunk[2], chunk[3]))
        .enumerate()
        .collect::<Vec<_>>();

    let maximum_brightness = color_converted
        .iter()
        .max_by_key(|&(_, c)| c.0)
        .map(|&(_, c)| c.0)
        .expect("something is max");

    let allowed_positions = color_converted
        .into_iter()
        .filter(|(_, c)| c.0 == maximum_brightness)
        .map(|(i, _)| (i % width, height - i / width))
        .collect::<Vec<_>>();

    Ok(thread_rng()
        .choose(&allowed_positions)
        .map(|&(x, y)| Point2::new(x as f32, y as f32)))
}

pub(crate) fn update(state: &mut MainState, ctx: &mut Context) -> GameResult<()> {
    state.frame += 1;

    if !state.zooming_in {
        state.port.zoom_out();

        if state.port.size.x > 6.4 {
            state.zooming_in = true;
        }
        return Ok(());
    }

    if state.port.size.x < 0.00004 {
        state.zooming_in = false;
    }

    if state.frame % 120 == 0 && state.port.cache.is_some() {
        let position = {
            let screenshot = state.port.cache.as_ref().expect("cache is some");
            select_zoom_position_screen(ctx, screenshot)?
        };

        if let Some(pos) = position {
            state.zoom_position = Some(state.port.screen_to_port(pos));
        }
    }

    if let Some(pos) = state.zoom_position {
        state.port.zoom_towards(pos);
    }

    Ok(())
}

pub(crate) fn draw(state: &mut MainState, ctx: &mut Context) -> GameResult<()> {
    state.port.draw(&state.shader, ctx)
}

pub(crate) fn interrupt(state: &mut MainState, ctx: &mut Context) {
    if state.frame > 10 {
        ctx.quit().unwrap();
    }
}
