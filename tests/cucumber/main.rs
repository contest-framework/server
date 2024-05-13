mod logic;
mod world;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use world::TertestrialWorld;

#[given(expr = "file {string} with content")]
async fn file_with_content(world: &mut TertestrialWorld, step: &Step, filename: String) {
  logic::create_file(
    &world.dir.as_ref().join(filename),
    step.docstring.as_ref().expect("no docstring"),
  )
  .await;
}

#[when("I start Tertestrial")]
async fn start_tertestrial(world: &mut TertestrialWorld) {
  logic::start_tertestrial(world).await;
}

#[then("it exits")]
async fn it_exits(world: &mut TertestrialWorld) {
  logic::wait_for_exit(world).await;
}

#[then("it prints")]
async fn it_prints(world: &mut TertestrialWorld, step: &Step) {
  logic::verify_prints(world, step.docstring.as_ref().unwrap().trim()).await;
}

#[when(expr = "receiving the command {string}")]
async fn client_sends_command(world: &mut TertestrialWorld, command: String) {
  logic::send_command(command, world.dir.as_ref()).await;
}

#[given(expr = "Tertestrial is running")]
async fn tertestrial_is_running(world: &mut TertestrialWorld) {
  logic::start_tertestrial(world).await;
  logic::verify_prints(world, "Tertestrial is online, Ctrl-C to exit").await;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  TertestrialWorld::cucumber()
    .fail_fast()
    // .max_concurrent_scenarios(Some(2))
    .run_and_exit("features/")
    .await;
}
