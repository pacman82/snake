extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston_window::keyboard::Key;
use rand::Rng;

const WIDTH: usize = 64;
const HEIGHT: usize = 35;

#[derive(Clone, Copy)]
enum Field {
    Empty,
    Border,
    Head,
    Tail(usize),
    Food,
}

impl Field {
    fn to_rgba(&self) -> [f32; 4] {
        match *self {
            Field::Empty => [0.0, 0.0, 0.0, 1.0],
            Field::Border => [0.0, 0.0, 1.0, 1.0],
            Field::Head => [1.0, 1.0, 1.0, 1.0],
            Field::Tail(_) => [1.0, 1.0, 1.0, 1.0],
            Field::Food => [0.0, 1.0, 0.0, 1.0],
        }
    }
}

struct World {
    direction: i32,
    head: usize,
    tail: usize,
    fields: [Field; WIDTH * HEIGHT],
}

fn main() {
    let mut world = World::new();
    let mut window: PistonWindow = WindowSettings::new("Snake",
                                                       [WIDTH as u32 * 20, HEIGHT as u32 * 20])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .expect("Could not open window");
    window.set_ups(8); //Updates Per Second
    while let Some(e) = window.next() {
        match e {
            Input::Update(_) => world.update(),
            Input::Press(Button::Keyboard(key)) => world.change_player_direction(key),
            Input::Render(render_args) => {
                let w = render_args.draw_width as f64 / WIDTH as f64;
                let h = render_args.draw_height as f64 / HEIGHT as f64;

                window.draw_2d(&e, |c, g| for (x, y, color) in world.fields
                    .iter()
                    .enumerate()
                    .map(|(index, &field)| (index % WIDTH, index / WIDTH, field.to_rgba())) {
                    let x = x as f64 * w;
                    let y = y as f64 * h;
                    rectangle(color, [x, y, x + w, y + h], c.transform, g);
                });
            }
            _ => (),
        }
    }
}

impl World {
    /// Setup new Game world. Sets up borders and empty space
    fn new() -> World {
        let mut fields = [Field::Empty; (WIDTH * HEIGHT) as usize];
        for x in 0..WIDTH {
            fields[x] = Field::Border;
            fields[(HEIGHT - 1) * WIDTH + x] = Field::Border;
        }
        for y in 0..HEIGHT {
            fields[y * WIDTH] = Field::Border;
            fields[y * WIDTH + WIDTH - 1] = Field::Border;
        }
        let head = (HEIGHT + 2) * WIDTH / 2;
        fields[head] = Field::Head;
        fields[head + 1] = Field::Tail(head);
        let mut ret = World {
            fields,
            direction: -1,
            head,
            tail: head + 1,
        };
        ret.fields[ret.random_empty()] = Field::Food;
        ret
    }

    fn update(&mut self) {

        if let Field::Head = self.fields[self.head] {
        } else {
            return;
        }

        // move head
        let mut new_head = (self.head as i32 + self.direction) as usize;
        if let Field::Tail(next) = self.fields[new_head] {
            if next == self.head {
                new_head = (self.head as i32 - self.direction) as usize;
            }
        }
        let new_head = new_head;

        self.fields[self.head] = Field::Tail(new_head);
        match self.fields[new_head] {
            Field::Empty => {
                self.fields[new_head] = Field::Head;
                self.head = new_head;

                // remove tail
                if let Field::Tail(new_tail) = self.fields[self.tail] {
                    self.fields[self.tail] = Field::Empty;
                    self.tail = new_tail;
                }
            }
            Field::Food => {
                self.fields[new_head] = Field::Head;
                self.head = new_head;
                self.fields[self.random_empty()] = Field::Food;
            }
            _ => (),
        }
    }

    fn change_player_direction(&mut self, key: Key) {
        self.direction = match key {
            Key::Up => -(WIDTH as i32),
            Key::Down => WIDTH as i32,
            Key::Left => -1,
            Key::Right => 1,
            _ => return,
        };
    }

    fn random_empty(&self) -> usize{
        let mut rng = rand::thread_rng();
        let mut index: usize;
        loop {
            index = rng.gen();
            index = index % (WIDTH * HEIGHT);
            match self.fields[index] {
                Field::Empty => break,
                _ => continue,
            }
        };
        index
    }
}
