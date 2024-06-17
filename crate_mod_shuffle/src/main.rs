//! in-dev main.rs for mod_shuffle

use derive_builder::Builder;
use mod_shuffle::{error::Result,
                  shuffle::{ModShuffler, ModShufflerBuilder}};

fn main() -> Result<()> {
    let new_made = ModShuffler::new(1, 2, 3, 3);
    println!("ModShuff w/ NEW: {:?}", new_made);

    let def_made = ModShuffler::default();
    println!("ModShuff w/ DEFAULT: {:?}", def_made);

    let build_made = ModShufflerBuilder::default().val(11)
                                                  .modulo(22)
                                                  .prime_forward_shuffler(7)
                                                  .prime_backward_shuffler(19)
                                                  .build()?;
    println!("ModShuff w/ BUILD: {:?}", build_made);

    Ok(())
}
