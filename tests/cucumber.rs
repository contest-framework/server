use cucumber::{given, then, when, World};
use tempfile::TempDir;

#[derive(Debug, World)]
// Accepts both sync/async and fallible/infallible functions.
#[world(init = Self::new)]
pub struct AnimalWorld {
  cat: Cat,
  workspace: TempDir,
}

impl AnimalWorld {
  fn new() -> Self {
    let root = tempfile::tempdir().unwrap();
    Self {
      cat: Cat { hungry: true },
      workspace: root,
    }
  }
}

#[given("Tertestrial is running")]
fn tertestrial_running(world: &mut AnimalWorld) {
  world
}

// -------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Cat {
  pub hungry: bool,
}

impl Cat {
  fn feed(&mut self) {
    self.hungry = false;
  }
}

#[given("a hungry cat")]
fn hungry_cat(world: &mut AnimalWorld) {
  world.cat.hungry = true;
}

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
    .max_concurrent_scenarios(Some(1))
    .run_and_exit("features/")
    .await;
}
