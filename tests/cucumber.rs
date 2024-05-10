use cucumber::{given, then, when, World};

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

// Don't forget to additionally `use cucumber::when;`.

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
  world.cat.feed();
}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
  assert!(!world.cat.hungry);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  AnimalWorld::cucumber()
    .fail_fast()
    .max_concurrent_scenarios(Some(2))
    .run_and_exit("features/")
    .await;
}
