///
/// This is an import. It imports something into scope
///
/// In this case, it's a `trait` which is akin to a
/// Java interface with a few differences.
///
/// We import traits like `rand::Rng` and `io::Write`
/// (Which is also a trait) to be able to use their
/// methods and associated functions on objects which
/// implement them. In other words, they must be in
/// scope to be usable.
///
use rand::Rng;
use std::io::Write;

///
/// An enumeration. This lists the colours we can use
///
/// In this case, this is similar to a Java enum, in
/// that there is no attached data to each variant.
///
/// An enum is a fully qualified type, meaning we can
/// implement traits for it, which in this case include
/// `Clone` (Cloneable), `Copy` (Can copy bitwise),
/// `Debug` (Displayable), `PartialEq` (`==` operator).
/// We implement these using a shorthand for auto code
/// generation called `derive`. It's pretty common in
/// rust.
///
#[derive(Clone, Copy, Debug, PartialEq)]
enum Colour {
    Red,
    Orange,
    Blue,
    White,
    Yellow,
    Green,
}

///
/// We implement the `str::FromStr` trait to be able
/// to parse a `Colour` from user input in an idiomatic
/// way.
///
/// A major difference with Java is that implementations
/// of things and interfaces/traits on things is that
/// they are declared separate from the thing's declaration.
///
/// For example, in Java you'd say
/// ```
/// public class Foo implements MyInterface extends MyClass, MyAbstractClass { /**/ }
/// ```
/// While in rust you'd say
/// ```
/// pub struct Foo { /* My data members */ }
/// impl MyTrait for Foo {
///     // MyTrait method implementations here.
/// }
/// impl Foo {
///     // Foo methods here.
/// }
/// ```
///
impl std::str::FromStr for Colour {
    ///
    /// This is the error type we return when we get an error.
    /// This is called an associated type, it's named by the
    /// trait.
    ///
    type Err = String;
    ///
    /// We take a string (`&str`) and spit out a `Result` which
    /// is either `Ok()` or `Err()`. This is error handling in rust.
    ///
    /// When we say `Self` we say the type which we're implementing
    /// for. In this case it's `Colour`.
    ///
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ///
        /// This could be called either lazyness or smartness. It imports
        /// the names of each of the colours into scope to allow use to
        /// omit `Colour::` before each name. For example, `Colour::Orange`
        /// becomes `Orange`.
        ///
        use Colour::*;
        //
        // A chain operation which essentially gets the first character
        // or, if it fails (Input is too short) will return early with
        // a message (The ? operator a few lines in means return early
        // if we get an `Err()` variant or strip the `Result` layer to
        // get the value (The `char`) back.
        //
        let first: char = text
            .chars()
            .next()
            .ok_or::<Self::Err>("Input too short!".into())?
            // Make it lowercase
            .to_ascii_lowercase();
        //
        // Rust's superpower `switch` statement.
        //
        match first {
            'r' => Ok(Red),    //
            'b' => Ok(Blue),   //
            'w' => Ok(White),  // All of these branches return a `Result`
            'y' => Ok(Yellow), // If they're okay with it.
            'g' => Ok(Green),  //
            'o' => Ok(Orange), //
            // In the case we get absolutely anything else, we return an error
            // telling us what we got instead.
            _ => Err(format!("Invalid initial character: `{}`", first)),
        }
    }
}

//
// This just enumerates the colours, for ease of use.
//
static COLOURS: &'static [Colour] = &[
    Colour::Red,
    Colour::Blue,
    Colour::White,
    Colour::Yellow,
    Colour::Green,
    Colour::Orange,
];

///
/// We make a state struct because we may want to
/// expose a gui layer, which isn't able to access
/// variables local to functions.
///
/// This has a single generic parameter `'a` which is
/// a lifetime. I can't summarize lifetimes in this
/// short project, but it should suffice to say it's
/// rust's gimmick to make it so fast and safe (As fast
/// as C++ and safe as GC languages as Java).
///
/// There's a nice entry in the rust book should you
/// want to explore more on this:
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
///
struct State<'a> {
    ///
    /// The pegs we're looking at right now.
    ///
    /// A `Vec` is the equivalent of a `List` or `ArrayList` in Java.
    ///
    pegs: Vec<Colour>,
    ///
    /// Previously chosen peg combinations, could be
    /// useful for logging events, etc. Currently unused
    /// other than to count number of attempts.
    ///
    pub previously_chosen: Vec<Vec<Colour>>,
    ///
    /// The previous games we've played (IE the previous
    /// states of `pegs`. This uses a tuple to denote the
    /// contents. Tuples are structures whose names are
    /// simply the types they contain. This one contains
    /// a list of colours, the number of tries it took
    /// (`usize` if a number type in rust, like `int` or
    /// `long` in Java), and whether it was won (`bool`)
    ///
    pub previous_games: Vec<(Vec<Colour>, usize, bool)>,
    ///
    /// Max number of pegs we can play with.
    ///
    size_pegs: usize,
    allow_duplicates: bool,
    ///
    /// We buffer the input, because it could be input
    /// over multiple lines or through a gui. Once the
    /// size reaches `size_pegs`, it will flush and try
    /// to finish a move.
    ///
    buffered_input: Vec<Colour>,
    ///
    /// Optionally describes the maximum number of moves
    /// in a game.
    ///
    max_tries: Option<usize>,
    ///
    /// A function pointer (AKA a variable that is a
    /// function) this is called when the player wins.
    ///
    win: Box<dyn Fn() + 'a>,
    ///
    /// Called when the player loses.
    ///
    lose: Box<dyn Fn() + 'a>,
    ///
    /// Terminal mode. Only outputs terminal prompts
    /// and messages if this is true.
    ///
    terminal: bool,
}

///
/// Implementing the state
///
impl<'a> State<'a> {
    ///
    /// A `new` function, akin to a constructor, will
    /// create a new `State` given some configuration
    /// parameters.
    ///
    /// Note that this may or may not return `Self` (
    /// AKA `State`) if it has invalid parameters.
    ///
    fn new(
        size: usize,
        allow_duplicates: bool,
        //
        // Equivalent of an `Integer` in Java which allows a
        // nullable `int`.
        //
        max_tries: Option<usize>,
        //
        // These functions use fancy talk in rust to say that
        // this function (new) is generic over some type which
        // implements the function traits. Kind of like this:
        // ```java
        // public static<T> void new(...) where T: MyInterface
        // ```
        //
        win: impl Fn() + 'a,
        lose: impl Fn() + 'a,
        terminal: bool,
    ) -> Option<Self> {
        //Check if there is a problem with our config
        if size > COLOURS.len() {
            if terminal {
                println!(
                    "Choose less than or equal to {} pegs to play with!",
                    COLOURS.len()
                )
            }
            // Return error state (`null` equivalent) if error
            None
        } else {
            let pegs = Self::generate_new_pegs(size, allow_duplicates);
            Some(
                // This is an inline constructor, we just specify
                // each field's value on declaration
                Self {
                    pegs,
                    previously_chosen: Vec::with_capacity(max_tries.clone().unwrap_or(0)),
                    previous_games: Vec::new(),
                    size_pegs: size,
                    allow_duplicates,
                    buffered_input: Vec::with_capacity(size),
                    max_tries,
                    win: Box::new(win),
                    lose: Box::new(lose),
                    terminal,
                },
            )
        }
    }

    ///
    /// This is an associated function which isn't run on anything. This
    /// is similar to Java's `static` methods except those have access to
    /// `static` state, which could be present in a class. These on the
    /// other hand cannot mutate anything outside of the function.
    ///
    /// From within this `impl` block, you call this as `Self::generate_new_pegs()`
    ///
    fn generate_new_pegs(size: usize, allow_duplicates: bool) -> Vec<Colour> {
        let mut rng = rand::thread_rng();
        // We use `.to_vec` so that we can remove items from it
        let mut choice_pegs = COLOURS.to_vec();
        if allow_duplicates {
            // This is equivalent to looping over `0` to `size` and
            // collecting these values into a list:
            //
            // choice_pegs[rng.gen::<usize>() % choice_pegs.len()]
            //
            (0..size)
                .map(|_| choice_pegs[rng.gen::<usize>() % choice_pegs.len()])
                .collect()
        } else {
            // This is the same idea except that instead of using indexing
            // we use removal, which will remove the item from the list and
            // return it.
            (0..size)
                .map(move |_| choice_pegs.remove(rng.gen::<usize>() % choice_pegs.len()))
                .collect()
        }
        // Because each branch of the if block ends with an implicit return
        // expression which isn't `void` (Or in rust, ()), the if statement
        // itself is now an expression which returns a value.
    }

    ///
    /// Returns either the number of correct placements followed by
    /// present colours or a string describing an error in the case
    /// it is not happy with its inputs.
    ///
    fn matching(&self, idx: Option<usize>) -> Result<(usize, usize), String> {
        let player = idx
            .map(|x| &self.previously_chosen[x])
            .unwrap_or(&self.buffered_input);
        if !self.allow_duplicates {
            let mut seen = Vec::new();
            player
                .iter()
                .enumerate()
                .try_fold((0, 0), |mut state, (idx, val)| {
                    if seen.contains(val) {
                        return Err("Cannot have duplicated when using non-duplicate mode!".into());
                    }
                    seen.push(*val);
                    if self.pegs.contains(val) {
                        if self.pegs[idx] == *val {
                            state.0 += 1;
                        } else {
                            state.1 += 1;
                        }
                    }
                    Ok(state)
                })
        } else {
            Ok(player
                .iter()
                .enumerate()
                .fold((0, 0), |mut state, (idx, val)| {
                    if self.pegs.contains(val) {
                        if self.pegs[idx] == *val {
                            state.0 += 1;
                        } else {
                            state.1 += 1;
                        }
                    }
                    state
                }))
        }
    }

    ///
    /// Pushes a colour into our buffered input, returning
    /// if a game change (Not a turn change) occurred.
    ///
    fn input_buffer(&mut self, value: Colour) -> Result<bool, String> {
        self.buffered_input.push(value);
        if self.buffered_input.len() == self.size_pegs {
            self.finish_try()
        } else {
            Ok(false)
        }
    }

    ///
    /// Parse and push a whole string as an input into the buffer.
    /// This uses `input_buffer` on every character in the string.
    ///
    fn push_string_input(&mut self, mut text: &str) -> Result<bool, (String, bool)> {
        let mut should_reset = false;
        while text.len() > 0 {
            //Intentionally ignoring the output because we can accept
            //strings longer than the max size and just keep processing
            //them to enter multiple tries at the same time.
            should_reset |= text
                .parse()
                .and_then(|x| self.input_buffer(x))
                .map_err(|x| (x, should_reset))?;
            // This line could break should we get a non-ascii character
            // but it should be fine for now
            text = &text[1..];
        }
        Ok(should_reset)
    }

    ///
    /// Decides to either win the game, or not, or keep going.
    ///
    fn finish_try(&mut self) -> Result<bool, String> {
        // `true` if we've finished a game, false if we've finished a round
        let returns;
        if self.buffered_input == self.pegs {
            // Call our function we assigned at the start if we win.
            // This currently just prints a "You win" message
            (self.win)();
            self.previous_games
                .push((self.pegs.clone(), self.previously_chosen.len(), true));
            self.reset();
            returns = true;
        } else {
            if self.max_tries.unwrap_or(std::usize::MAX) == self.previously_chosen.len() + 1 {
                (self.lose)();
                self.previous_games
                    .push((self.pegs.clone(), self.previously_chosen.len(), false));
                self.reset();
                returns = true;
            } else {
                if self.terminal {
                    let matching = self.matching(None)?;
                    println!(
                        "Good try, here are your matching pegs: {:?} are in the correct position and {:?} have the right colour",
                        matching.0,
                        matching.1,
                    )
                }
                self.previously_chosen
                    .push(self.buffered_input.drain(..).collect());
                returns = false;
            }
        }
        Ok(returns)
    }

    fn reset(&mut self) {
        self.previously_chosen = Vec::new();
        self.buffered_input.clear();
        self.pegs = Self::generate_new_pegs(self.size_pegs, self.allow_duplicates);
    }
}

///
/// A mock main, meant to be copy-pasteable into other places.
///
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // There are three ways to write a string in rust,
    // "this way", r#"this way"#, and r"this way".
    // The first one is your standard string with escape
    // sequences like \n, etc. The second one is to ignore
    // all characters between the #"s and just take them
    // as if they were text. r"" text is a byte array literal
    // instead of a string.
    println!(
        r#"
    ~~~~ Mastermind ~~~~
Rules: A set of pegs from the
following colours are selected:
   ┏━━━━━━┳━━━━━━┳━━━━━━┓
   ┃Orange┃Yellow┃ Red  ┃
   ┣━━━━━━╋━━━━━━╋━━━━━━┫
   ┃ Blue ┃Green ┃White ┃
   ┗━━━━━━┻━━━━━━┻━━━━━━┛
The player takes guesses at
the selected colours, and is
given the number of pegs in a
correct position (And colour)
and the number of correct
colours chosen in an incorrect
position.
"#
    );
    let mut input = String::new();
    print!("Would you like to allow duplicates? (\"true\" or \"false\"): ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    let mut duplicates = input.trim().parse::<bool>();
    let duplicates = loop {
        match duplicates {
            Ok(x) => break x,
            Err(_) => {
                println!("Please try again! Either `true` or `false`.");
                input.clear();
                std::io::stdin().read_line(&mut input)?;
                duplicates = input.trim().parse();
            }
        }
    };
    input.clear();
    print!("How many pegs would you like to play with? (2-6, inclusive): ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    let mut pegs = input.trim().parse::<usize>();
    let pegs = loop {
        match pegs {
            Ok(x @ 2..=6) => break x,
            _ => {
                println!("Please try again! Enter a valid positive integer from 2-6 inclusive.");
                input.clear();
                std::io::stdin().read_line(&mut input)?;
                pegs = input.trim().parse();
            }
        }
    };

    // Here we use our new function above.
    let mut state = State::new(
        pegs,
        duplicates,
        Some(10),
        || println!("You won!"),
        || println!("Uh-oh, you lost"),
        true,
    )
    .unwrap();

    for i in 0..2 {
        println!("Generated new state! Game #{}", i + 1);
        'a: loop {
            print!("Enter next colours > ");
            std::io::stdout().flush()?;
            input.clear();
            std::io::stdin().read_line(&mut input)?;
            match state.push_string_input(input.trim()) {
                Ok(f) => {
                    if f {
                        break 'a;
                    }
                }
                Err((text, f)) => {
                    println!("Error encountered: {}", text);
                    if f {
                        break 'a;
                    }
                }
            }
        }
    }

    println!("Previous games:");
    for (idx, (pegs, attempts, won)) in state.previous_games.iter().enumerate() {
        println!(
            "Game #{} with pegs {:?} was {} with {} attempts",
            idx + 1,
            pegs,
            if *won { "won" } else { "lost" },
            attempts
        );
    }
    Ok(())
}
