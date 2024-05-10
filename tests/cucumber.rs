use cucumber::{given, World};

#[derive(Debug, World)]
// Accepts both sync/async and fallible/infallible functions.
#[world(init = Self::new)]
pub struct AnimalWorld {
    cat: Cat,
}

impl AnimalWorld {
    fn new() -> Self {
        Self {
            cat: Cat { hungry: true },
        }
    }
}

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a hungry cat")]
fn hungry_cat(world: &mut AnimalWorld) {
    world.cat.hungry = true;
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(AnimalWorld::run("features/"));
}
