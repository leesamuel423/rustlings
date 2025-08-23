// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}

/*
 * semicolon -> expression vs statement
 * - adding semicolon turns expression (returns a value) to a statement (performs action but
 * doesn't return anything (technically (), the unit type)
 *
 * So general rule:
 * - Last line of fn that returns a value, no semicolon
 * - All other lines usually have semicolons b/c they're statements
 */
