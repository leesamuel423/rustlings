// TODO: Change the line below to fix the compiler error.
const NUMBER: u32 = 3;

fn main() {
    println!("Number: {NUMBER}");
}

/*
 * `const` requires explicit type annotation
 * - Evaluated at compile time and used in many contexts
 * - They can be used across module boundaries where inference would be ambiguous
 * - Explicit types makes API contract clearer
 * - naming convention is SCREAMING_SNAKE_CASE
 * - no fixed mem address - inlined wherever used
 * - always immutable
 *
 * `const` is compile time constant, `let` is runtime variable
 *
 * Use const for configuration variables, constants, array sizes. Aka variables that never change
 * and are known at compile time
 */
