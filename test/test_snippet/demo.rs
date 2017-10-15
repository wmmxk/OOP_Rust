// This code is to show how to test a custom function.
// This is similar to Python, in python you can test a module without a main function
//  To compile: rustc demo.rs --test

//
enum Answer {
    Higher,
    Lower,
    Bingo,
}

fn suggest_guess(prior_guess: u32, answer: Answer) {
    match answer {
        Answer::Higher => println!("maybe try {} next", prior_guess + 10),
        Answer::Lower  => println!("maybe try {} next", prior_guess - 1),
        Answer::Bingo  => println!("we won with {}!", prior_guess),
    }
}

#[test]
fn demo_suggest_guess() {
    suggest_guess(10, Answer::Higher);
    suggest_guess(20, Answer::Lower);
    suggest_guess(19, Answer::Bingo);
}
