//! in-dev main.rs for mod_shuffle

use derive_builder::Builder;
use mod_shuffle::{error::Result,
                  shuffle::{ProposedModShuffler, ProposedModShufflerBuilder}};

fn main() -> Result<()> {
    let new_made = ProposedModShuffler::new(1, 2, 3, 3);
    let new_made = new_made.validate()?;
    println!("ModShuff w/ NEW: {:?}", new_made);

    let def_made = ProposedModShuffler::default();
    let def_made = def_made.validate()?;
    println!("ModShuff w/ DEFAULT: {:?}", def_made);

    let build_made = ProposedModShufflerBuilder::default().val(10)
                                                          .modulo(22)
                                                          .prime_forward_shuffler(7)
                                                          .prime_backward_shuffler(19)
                                                          .build()?;
    let build_made = build_made.validate()?;
    println!("ModShuff w/ BUILD: {:?}", build_made);
    println!("-------------------------------------------");

    let new2 = new_made.fwd_shuffle();
    let def2 = def_made.fwd_shuffle();
    let build2 = build_made.fwd_shuffle();
    println!("new2 {:?}", new2);
    println!("def2 {:?}", def2);
    println!("build2 {:?}", build2);
    println!("-------------------------------------------");

    let new3 = new2.rev_shuffle();
    let def3 = def2.rev_shuffle();
    let build3 = build2.rev_shuffle();
    println!("new3 {:?}", new3);
    println!("def3 {:?}", def3);
    println!("build3 {:?}", build3);
    println!("-------------------------------------------");

    Ok(())
}
