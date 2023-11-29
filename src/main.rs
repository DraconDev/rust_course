use learn_rust_book::{
    collections::{hash_maps, vectors},
    panic_lesson::{file_practice, panicky},
};

mod learn_rust_book;
mod leetcode;
mod my_mod;
mod test_modules;

fn regex_test() {
    use regex::Regex;
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}

fn main() {
    // hash_maps();
    // panicky();
    file_practice();

    // let mut s = String::from("foo");

    // s.push_str("bar");
    // function();
    // regex_test();
    // vectors();

    // * import from module
    // my_mod::function();
    // test_modules::say_hi();
    // learn_rust_book::functions::print_labeled_measurement(5, 'h');

    // * import from lib
    // rust_course::front_of_house::hosting::add_to_waitlist();
}

fn function() {
    println!("called `function()`");
}
