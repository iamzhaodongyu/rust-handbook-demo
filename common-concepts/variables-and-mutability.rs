fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constant differences with mut
    // First, you aren't allowed to use mut with constants
    // Constatants aren't just immutable by default -- they are always immutable
    // Second, The type of the value must be annotated
    // The last difference is that constants may be set only to a constant expression
    // not the result of a value that could only be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code
    // It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.

    println!("The value of constant identifer: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing is different from making a variable as mut,
    // because we will get compile-time error if we accidentally try to reassign to this variable without using the let keyword
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed

    // The other difference between mut and shaowding is that because we are effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name

    let spaces = "  ";
    let spaces = spaces.len();

    let mut another_spaces = "  ";
    another_spaces = another_spaces.len();
}