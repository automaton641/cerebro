use rand::prelude::*;
use rand::rngs::ThreadRng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::fmt;
use std::thread;
use std::time::Duration;

struct Vector2d {
    x: i128,
    y: i128,
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct ParticleType {
    color: Color,
    charge: Vector2d,
}

struct Particle {
    already_moved: bool,
    velocity: Vector2d,
    particle_type: usize,
}
struct Cell {
    particle: Option<Particle>,
    field: Vector2d,
}

struct Cerebro {
    particle_types: Vec<ParticleType>,
    width: i128,
    height: i128,
    space: Vec<Vec<Cell>>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            particle: None,
            field: Vector2d { x: 0, y: 0 },
        }
    }
}

fn do_nothing() {}

impl Cerebro {
    fn new(width: i128, height: i128) -> Cerebro {
        let mut cerebro = Cerebro {
            particle_types: Vec::new(),
            width: width,
            height: height,
            space: Vec::new(),
        };
        for x in 0..width {
            let column: Vec<Cell> = Vec::new();
            cerebro.space.push(column);
            for _y in 0..height {
                cerebro.space[x as usize].push(Cell::new());
            }
        }
        return cerebro;
    }
    fn place_particles(&mut self, rng: &mut ThreadRng) {
        let mut iteration = 0;
        let spacing = 4;
        for x in 0..self.width / 2 {
            for y in 0..self.height {
                if iteration == 0 {
                    let random: usize = rng.gen();
                    let modulus: usize = self.particle_types.len();
                    self.space[x as usize][y as usize].particle =
                        Some(Particle::new(random % modulus));
                }
                iteration += 1;
                iteration %= spacing;
            }
            iteration += 1;
            iteration %= spacing;
        }
    }
    fn reset_fields(&mut self) {
        for column in &mut self.space {
            for cell in column {
                cell.field.x = 0;
                cell.field.y = 0;
            }
        }
    }

    fn get_cell(&mut self, x: i128, y: i128) -> &mut Cell {
        let mut xi = x;
        while xi < 0 {
            xi += self.width;
        }
        while xi >= self.width {
            xi -= self.width;
        }
        let mut yi = y;
        while yi < 0 {
            yi += self.height;
        }
        while yi >= self.height {
            yi -= self.height;
        }
        &mut self.space[xi as usize][yi as usize]
    }

    fn apply_charge(&mut self, particle_type: usize, x: i128, y: i128) {
        let charge_x_init = self.particle_types[particle_type].charge.x;
        for xi in 0..charge_x_init.abs() {
            let charge_x = if charge_x_init < 0 {
                charge_x_init + xi
            } else {
                charge_x_init - xi
            };
            for dx in -xi..=xi {
                for dy in -xi..=xi {
                    let cell = self.get_cell(x + dx, y + dy);
                    cell.field.x += charge_x;
                }
            }
        }
        let charge_y_init = self.particle_types[particle_type].charge.y;
        for yi in 0..charge_y_init.abs() {
            let charge_y = if charge_y_init < 0 {
                charge_y_init + yi
            } else {
                charge_y_init - yi
            };
            for dx in -yi..=yi {
                for dy in -yi..=yi {
                    let cell = self.get_cell(x + dx, y + dy);
                    cell.field.y += charge_y;
                }
            }
        }
    }
    fn apply_charges(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut particle_type: usize = 0;
                let mut has_particle: bool = false;
                match &self.space[x as usize][y as usize].particle {
                    Some(particle) => {
                        has_particle = true;
                        particle_type = particle.particle_type;
                    }
                    None => do_nothing(),
                }
                if has_particle {
                    self.apply_charge(particle_type, x, y);
                }
            }
        }
    }
    fn move_particles(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut has_particle = false;
                {
                    let cell = self.get_cell(x, y);
                    let particle = &mut cell.particle;
                    match particle {
                        Some(particle) => {
                            if !particle.already_moved {
                                has_particle = true;
                                particle.already_moved = true;
                            }
                        }
                        None => do_nothing(),
                    }
                }
                if has_particle {
                    let field_x: i128;
                    let field_y: i128;
                    {
                        let cell = self.get_cell(x, y);
                        field_x = cell.field.x;
                        field_y = cell.field.y;
                    }
                    let mut can_move: bool = false;
                    match &self.get_cell(x + field_x, y + field_y).particle {
                        Some(_particle) => do_nothing(),
                        None => {
                            can_move = true;
                        }
                    }
                    if can_move {
                        let particle = self.get_cell(x, y).particle.take();
                        self.get_cell(x + field_x, y + field_y).particle = particle;
                    }
                }
            }
        }
    }
    fn make_particles_movable_again(&mut self) {
        for column in &mut self.space {
            for cell in column {
                match &mut cell.particle {
                    Some(particle) => {
                        particle.already_moved = false;
                    }
                    None => do_nothing(),
                }
            }
        }
    }
    fn iterate(&mut self) {
        self.reset_fields();
        self.apply_charges();
        self.move_particles();
        self.make_particles_movable_again();
    }
}

impl ParticleType {
    fn new(x: i128, y: i128, red: u8, green: u8, blue: u8) -> ParticleType {
        ParticleType {
            charge: Vector2d { x: x, y: y },
            color: Color {
                red: red,
                green: green,
                blue: blue,
            },
        }
    }
}

impl Particle {
    fn new(particle_type: usize) -> Particle {
        Particle {
            already_moved: false,
            velocity: Vector2d { x: 0, y: 0 },
            particle_type: particle_type,
        }
    }
}

impl fmt::Display for Vector2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "red {} green {} blue {}",
            self.red, self.green, self.blue
        )
    }
}

impl fmt::Display for ParticleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "charge {} color {}", self.charge, self.color)
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "velocity {} type {}", self.velocity, self.particle_type)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.particle {
            Some(particle) => write!(f, "(particle {}) (field {})", particle, self.field),
            None => write!(f, "(particle none) (field {})", self.field),
        }
    }
}

fn draw_rectangle(
    x: usize,
    y: usize,
    pixels: &mut Vec<u8>,
    width: usize,
    color: &Color,
    cell_size: usize,
) {
    let x_init = x * cell_size;
    let x_final = x_init + cell_size;
    let y_init = y * cell_size;
    let y_final = y_init + cell_size;
    for x_pixel in x_init..x_final {
        for y_pixel in y_init..y_final {
            let index = y_pixel * width * 4 + x_pixel * 4;
            pixels[index as usize] = color.blue;
            pixels[index as usize + 1] = color.green;
            pixels[index as usize + 2] = color.red;
            pixels[index as usize + 3] = 255;
        }
    }
}

fn draw_cerebro(cerebro: &Cerebro, pixels: &mut Vec<u8>, width: usize, cell_size: usize) {
    let mut x = 0;
    for column in &cerebro.space {
        let mut y = 0;
        for cell in column {
            match &cell.particle {
                Some(particle) => draw_rectangle(
                    x,
                    y,
                    pixels,
                    width,
                    &cerebro.particle_types[particle.particle_type].color,
                    cell_size,
                ),
                None => draw_rectangle(
                    x,
                    y,
                    pixels,
                    width,
                    &Color {
                        red: 255,
                        green: 255,
                        blue: 255,
                    },
                    cell_size,
                ),
            }
            y += 1;
        }
        x += 1;
    }
}

fn main() {
    let mut cerebro = Cerebro::new(128, 64);
    //cerebro.space[0][0].particle = Some(Particle::new(0));
    //cerebro.space[4][0].particle = Some(Particle::new(1));
    let max_deviation: i128 = 2;
    let mut combinations: i128 = 0;
    for _x in -max_deviation..=max_deviation {
        for _y in -max_deviation..=max_deviation {
            combinations += 1;
        }
    }
    let mut combination: i128 = 0;
    for x in -max_deviation..=max_deviation {
        for y in -max_deviation..=max_deviation {
            let color: u64 = combination as u64 * (16777216 / combinations as u64);
            let red: u8 = color as u8;
            let green: u8 = (color >> 8) as u8;
            let blue: u8 = (color >> 16) as u8;
            cerebro
                .particle_types
                .push(ParticleType::new(x, y, red, green, blue));
            combination += 1;
        }
    }
    let mut rng = thread_rng();
    cerebro.place_particles(&mut rng);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let cell_size: usize = 8;
    let width: u32 = cerebro.width as u32 * cell_size as u32;
    let height: u32 = cerebro.height as u32 * cell_size as u32;
    let window = video_subsystem
        .window("Juego", width, height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, width, height)
        .unwrap();
    let mut pixels: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 4);
    for _index in 0..width * height {
        //BGRA
        pixels.push(0xff);
        pixels.push(0xff);
        pixels.push(0xff);
        pixels.push(0xff);
    }
    let texture_rectangle = Rect::new(0, 0, width, height);
    texture
        .update(texture_rectangle, &pixels, width as usize * 4)
        .expect("texture.update");
    canvas
        .copy(&texture, texture_rectangle, texture_rectangle)
        .expect("canvas.copy");
    canvas.present();
    let mut pre_iteration = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        if pre_iteration == 0 {
            //println!("iteration");
            draw_cerebro(&cerebro, &mut pixels, width as usize, cell_size);
            texture
                .update(texture_rectangle, &pixels, width as usize * 4)
                .expect("texture.update");
            canvas
                .copy(&texture, texture_rectangle, texture_rectangle)
                .expect("canvas.copy");
            canvas.present();
            cerebro.iterate();
        }
        pre_iteration += 1;
        pre_iteration %= 1;
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 64));
    }
}
