use crate::UserError;

pub fn print_error(err: &UserError) {
  let (msg, desc) = err.messages();
  println!("{msg}");
  println!("{desc}");
}
