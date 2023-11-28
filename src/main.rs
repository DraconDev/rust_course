mod my_mod;
mod test_modules;
fn main() {
    function();
    my_mod::function();
    test_modules::say_hi()
}

fn function() {
    println!("called `function()`");
}
