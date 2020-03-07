pub trait GameLoop {
    fn fixed_update_delta(&self) -> f64 {
        1.0 / 60.0
    }

    fn is_running(&self) -> bool {
        true
    }

    fn run(&mut self) {
        let instant = std::time::Instant::now();
        let mut physics_tick_count: f64 = 0.0;
        let mut last_count = instant.elapsed().as_secs_f64();
        let mut delta;
        while self.is_running() {
            let now = instant.elapsed().as_secs_f64();
            delta = now - last_count;
            last_count = now;
            physics_tick_count += delta;

            self.process_events();
            if physics_tick_count > self.fixed_update_delta() {
                self.fixed_update(self.fixed_update_delta());
                physics_tick_count -= self.fixed_update_delta();
            }
            self.update(delta);
            self.render();
        }
    }

    fn process_events(&mut self);
    fn fixed_update(&mut self, delta: f64);
    fn update(&mut self, delta: f64);
    fn render(&self);
}
