extern crate english_lint;
use english_lint::{Hint, lint};

#[test]
fn example_1() {
    let text = "Before we get to the details, two important notes about the ownership system.

Rust has a focus on safety and speed. It accomplishes these goals through many ‘zero-cost abstractions’, which means that in Rust, abstractions cost as little as possible in order to make them work. The ownership system is a prime example of a zero-cost abstraction. All of the analysis we’ll talk about in this guide is done at compile time. You do not pay any run-time cost for any of these features.

However, this system does have a certain cost: learning curve. Many new users to Rust experience something we like to call ‘fighting with the borrow checker’, where the Rust compiler refuses to compile a program that the author thinks is valid. This often happens because the programmer’s mental model of how ownership should work doesn’t match the actual rules that Rust implements. You probably will experience similar things at first. There is good news, however: more experienced Rust developers report that once they work with the rules of the ownership system for a period of time, they fight the borrow checker less and less.

With that in mind, let’s learn about lifetimes.";

    assert_eq!(lint(text), vec![
        Hint { group: Some("wordy"), value: "accomplish".to_owned(), line: Some(3), start: 41, end: 51 },
        Hint { group: Some("adverbs"), value: "many".to_owned(), line: Some(3), start: 74, end: 78 },
        Hint { group: Some("wordy"), value: "in order to".to_owned(), line: Some(3), start: 175, end: 186 },
        Hint { group: Some("wordy"), value: "all of".to_owned(), line: Some(3), start: 271, end: 277 },
        Hint { group: Some("passive"), value: "is done".to_owned(), line: Some(3), start: 324, end: 331 },
        Hint { group: Some("wordy"), value: "however".to_owned(), line: Some(5), start: 0, end: 7 },
        Hint { group: Some("adverbs"), value: "many".to_owned(), line: Some(5), start: 63, end: 67 },
        Hint { group: Some("wordy"), value: "implement".to_owned(), line: Some(5), start: 380, end: 389 },
        Hint { group: Some("weakens"), value: "things".to_owned(), line: Some(5), start: 429, end: 435 },
        Hint { group: Some("wordy"), value: "however".to_owned(), line: Some(5), start: 466, end: 473 }
    ]);
}

#[test]
fn example_2() {
    let text = "So you’ve learned how to write some Rust code. But there’s a difference between writing any Rust code and writing good Rust code.

This chapter consists of relatively independent tutorials which show you how to take your Rust to the next level. Common patterns and standard library features will indeed be introduced. Read these sections in any order of your choosing.";

    assert_eq!(lint(text), vec![
        Hint { group: Some("adverbs"), value: "relatively".to_owned(), line: Some(3), start: 25, end: 35 },
        Hint { group: Some("passive"), value: "be introduced".to_owned(), line: Some(3), start: 172, end: 185 }
    ]);
}
