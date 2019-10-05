mod base;
mod solvers;

pub use base::Number;
pub use base::functions::*;
pub use solvers::caesar;
pub use solvers::vigenere;
pub use solvers::aes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod challenges;