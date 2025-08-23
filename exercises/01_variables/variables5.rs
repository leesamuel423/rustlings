fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

/*
 * Not reassigning variable in the code, but creating new variable w/ same name that
 * "shadows" (hides) previous one.
 *
 * `let` is being used both times, indicating creation of new variable
 *
 * Shadowing vs reassignment
 * SHADOWING
 * - For shadowing, uses `let` to declare new variable w/ same name.
 * - Can change type (string → int)
 * - original variable still exists but is no longer accessible by the name
 * REASSIGNMENT
 * - requires var to be declared as mutable
 * - can't change type
 *
 * Shadowing is nice cause you can transform values while keeping semantically meaningful name.
 * Change types when needed.
 * Variables are immutable after transforming them.
 *
 * Rust DOES clean up original value for you. If original value can't be referenced, Rust
 * drops it (free up memory) as soon as it's shadowed.
 * Rust is using ownership system and scope rules:
 *  - When value has no more valid references, dropped
 *  - Happens automatically, no garbage collector needed.
 *  - Compiler tracks this at compile time
 *  - This is part of rust's "zero-cost abstractions" - automatic memory management w/o runtime
 *  overhead
 */
