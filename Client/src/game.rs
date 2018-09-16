pub struct Game {
    running: bool
}

impl Game {
    pub fn new() -> Game {
        let running = true;

        Game {
            running
        }
    }

    pub fn handle_input(&self) -> bool {
        return true;
    }

    pub fn run(&mut self) {
        while self.running {
            self.running = self.handle_input();
            println!("Dit is een test.")
        }
    }
}