//!
//! Hello! If you are reading this, then that means you
//! have received a copy of my `Mastermind` implementation.
//!
//! I will walk you through my implementation and add notes
//! where necessary to explain things specific to Rust as a
//! language which is different from Java.
//!

///
/// This is a module declaration, for the main mastermind
/// implementation. In rust, the main module, referred to
/// as `crate` (for example it can be used as `crate::main`
/// to call the main function below). It owns more modules
/// each of which can own more modules, therefore creating
/// bit of a module tree. In this case, `crate` owns one
/// module, called `mastermind`.
///
mod mastermind;

///
/// The main method, AKA the entrypoint of the program.
/// This is a global function, because there is no state
/// (Or static state) associated with the global entrypoint
///
/// This function returns (Denoted with the arrow), a
/// `Result<(), Box<dyn Error>>`. This means that it will
/// return either nothing (On success, represented by the
/// `()`, equivalent to Java's `void`) or, on an error,
/// a boxed up `Error`, which essentially means, it will
/// print an error should it fail to run.
///
/// Returning a result is similar to Java's `throws Exception`,
/// it allows us to propagate an error.
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // The `mastermind::main` function returns the same
    // type as this function, and because the last statement
    // of a block of code (AKA surrounded with curly braces
    // `{}`) is returned from that block if it doesn't end
    // with a semicolon
    //
    mastermind::main()
}

//
// To continue please navigate to the mastermind directory
// and read the `mod.rs` file. This is what we declared
// above.
//
