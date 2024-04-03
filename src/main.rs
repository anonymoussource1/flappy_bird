use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf;
use sdl2::ttf::Font;
use std::thread;
use std::time::Duration;
use std::ops::Deref;
use rand::prelude::*;

const PLAYER_SIZE_MUL: u32 = 4;
const PIPE_SIZE_MUL: u32 = 3;
const SCREEN_WIDTH: u32 = 1000;
const SCREEN_HEIGHT: u32 = 800;
const HOLE_HEIGHT: u32 = 200;
const GROUND_HEIGHT: u32 = 100;

struct Player {
    bounding_box: Rect,
    is_dead: bool,
    frames_falling: u8,
}

struct Pipe {
    bounding_box: Rect,
    hole_y: i32,
    has_scored: bool,
}

struct Button<'a, F> where F: FnMut() {
    rect: Rect,
    font: &'a Font<'a, 'a>,
    text: &'static str,
    on_click: F,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            bounding_box: Rect::new(x, y, 16 * PLAYER_SIZE_MUL, 16 * PLAYER_SIZE_MUL - 4 * PLAYER_SIZE_MUL),
            is_dead: false,
            frames_falling: 0,
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let bird = texture_creator.load_texture("bird.png")?;
        canvas.copy(&bird, Some(Rect::new(0, 0, 16, 16)), Some(Rect::new(self.x(), self.y(), self.width(), self.height() + 4 * PLAYER_SIZE_MUL)))?;
        Ok(())
    }
}

impl Deref for Player {
    type Target = Rect;

    fn deref(&self) -> &Self::Target {
        &self.bounding_box
    }
}

impl Pipe {
    fn new(x: i32) -> Self {
        Pipe {
            bounding_box: Rect::new(x, 0, 64, 700),
            hole_y: Self::generate_hole(),
            has_scored: false,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let pipe = texture_creator.load_texture("pipe.png")?;
        let TextureQuery { width, height, .. } = pipe.query();
        let width = width * PIPE_SIZE_MUL;
        let height = height * PIPE_SIZE_MUL;

        canvas.copy(&pipe, None, Some(Rect::new(self.x(), self.hole_y - height as i32, width, height)))?;
        canvas.copy(&pipe, None, Some(Rect::new(self.x(), self.hole_y + HOLE_HEIGHT as i32, width, height)))?; 

        Ok(())
    }

    fn generate_hole() -> i32 {
        thread_rng().gen_range(50..=450)
    }
}

impl Deref for Pipe {
    type Target = Rect;

    fn deref(&self) -> &Self::Target {
        &self.bounding_box
    }
}

impl<'a, F> Button<'a, F> where F: FnMut() -> () {
    fn new(x: i32, y: i32, font: &'a Font<'a, 'a>, text: &'static str, on_click: F) -> Self {
        Button {
            rect: Rect::new(x, y, 0, 0),
            font,
            text,
            on_click,
        }
    }

    fn check_for_click<P>(&mut self, point: P) where P: Into<(i32, i32)> {
        if self.rect.contains_point(point) {
            (self.on_click)();
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let text_surface = self.font.render(self.text)
                                    .blended(Color::RGB(0, 0, 0))
                                    .map_err(|e| e.to_string())?;
        let text_texture = texture_creator.create_texture_from_surface(&text_surface)
                                            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = text_texture.query();
        self.rect.set_width(width + 10);
        self.rect.set_height(height + 10);
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(self.rect)?;
        canvas.copy(&text_texture, None, Some(Rect::new(self.rect.x() + 5, self.rect.y() + 5, self.rect.width(), self.rect.height())))?;
        Ok(())
    }
}

fn angle(x: i32) {
    let slope = x as f64 / 14 + 1 / 7;


fn main() -> Result<(), String> {
    let sdl2_context = sdl2::init()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl2_context.video()?;
    let _image_subsystem = sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem
        .window("Spinny Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut score = 0;
    let font = ttf_context.load_font("Courier New Bold.ttf", 45)?;

    canvas.set_draw_color(Color::RGB(125, 255, 125));
    canvas.clear();

    let mut player = Player::new(200, 400);
    let mut frames = 0;

    let mut pipes: Vec<Pipe> = Vec::new();
    for x in 0..3 {
        pipes.push(Pipe::new(SCREEN_WIDTH as i32 / 2 + x * 400));
    }

    let mut button = Button::new(50, 50, &font, "Click me!", || println!("Clicked!"));
    let mut is_jump_key_down = false;
    let mut event_pump = sdl2_context.event_pump()?;
    'running: loop {
        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } if !is_jump_key_down => {
                    player.frames_falling = 0;
                    is_jump_key_down = true;
                }
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => is_jump_key_down = false,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. }  => button.check_for_click((x, y)),
                _ => {},
            }
        }

        player.bounding_box.y += ((player.frames_falling as f32 / 5f32 + 2f32).powi(2) / 28f32) as i32 - 3;
        if player.y() > SCREEN_HEIGHT - GROUND_HEIGHT - player.height() as i32 {
            player.is_dead = true;
            break 'running;
        }

        // Draw background
        canvas.set_draw_color(Color::RGB(125, 125, 255));
        canvas.clear();
        // Draw and update pipes
        for pipe in pipes.iter_mut() {
            pipe.bounding_box.x -= 1 + frames / 900;
            if player.bounding_box.has_intersection(Rect::new(pipe.x(), pipe.hole_y + HOLE_HEIGHT as i32, pipe.width(), SCREEN_HEIGHT - GROUND_HEIGHT - (pipe.hole_y as u32 + HOLE_HEIGHT))) || player.bounding_box.has_intersection(Rect::new(pipe.x(), 0, pipe.width(), pipe.hole_y as u32)) {
                player.is_dead = true;
                break 'running;
            }
            if !pipe.has_scored && pipe.has_intersection(*player) {
                score += 1;
                pipe.has_scored = true;
            }
            if pipe.x() < -200 {
                pipe.bounding_box.x = 1000;
                pipe.hole_y = Pipe::generate_hole();
                pipe.has_scored = false;
            }
            pipe.draw(&mut canvas)?;
        }
        
        // Draw bird
        player.draw(&mut canvas)?;
        // Draw floor
        canvas.set_draw_color(Color::RGB(125, 255, 125));
        canvas.fill_rect(Rect::new(0, SCREEN_HEIGHT as i32 - GROUND_HEIGHT as i32, SCREEN_WIDTH, GROUND_HEIGHT))?;
        // Draw score
        let font_surface = font.render(&format!("{score}"))
                                .blended(Color::RGB(0, 0, 0))
                                .map_err(|e| e.to_string())?;
        let font_texture = texture_creator.create_texture_from_surface(&font_surface)
                                            .map_err(|e| e.to_string())?;
        let TextureQuery { width: font_width, height: font_height, .. } = font_texture.query();
        canvas.copy(&font_texture, None, Some(Rect::new((1000 / 2) - (font_width as i32 / 2), 25, font_width, font_height)))?;
        button.draw(&mut canvas)?;
        canvas.present();

        thread::sleep(Duration::from_millis(1000/60));
        frames += 1;
        player.frames_falling += 1;
    }

    Ok(())
}
