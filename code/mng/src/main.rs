use mng::engine::GameLoop;

struct MyGameLoop {
    running: bool,
}

impl MyGameLoop {
    pub fn new() -> MyGameLoop {
        MyGameLoop { running: true }
    }
}

impl GameLoop for MyGameLoop {
    fn is_running(&self) -> bool {
        self.running
    }
    fn process_events(&mut self) {
        println!("MyGameLoop::process_events()");
    }
    fn fixed_update(&mut self, delta: f64) {
        println!("MyGameLoop::fixed_update({})", delta);
    }
    fn update(&mut self, delta: f64) {
        println!("MyGameLoop::update({})", delta);
    }
    fn render(&self) {
        println!("MyGameLoop::render()");
    }
}

fn main() {
    let mut game_loop = MyGameLoop::new();
    game_loop.run();
}
