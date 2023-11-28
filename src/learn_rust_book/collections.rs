pub fn vectors() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // let does_not_exist2 = v.get(100);
}

/* In Rust, String and &str (commonly referred to as a string slice) are both used to handle string data, but they have key differences:

String:
String is a growable, mutable, owned, heap-allocated data structure. This means you have more flexibility with a String because you can change it by adding, removing, or updating characters.
It is implemented as a wrapper over a Vec<u8>. When you modify a String, you're modifying the underlying Vec<u8>.
Because a String is owned, when it goes out of scope, Rust will automatically deallocate the memory it was using.


let mut s = String::from("hello");
s.push_str(", world!");  // s is now "hello, world!"
&str (String Slice):
&str is a reference to a string. It is usually seen in the form of a string literal like "hello", which is of type &'static str. Because it's a reference, it's immutable by default.
&str is a view into a String. It doesn't own the string data; instead, it borrows the data from the owner.
&str is very memory efficient, as it only needs to store the starting position and length of the slice, not the data itself.

let s = "hello, world!";  // s is a &str
To summarize, a String is an owned buffer of UTF-8 bytes, while a &str is a read-only view into a well-formed UTF-8 sequence. */

/* String in Rust is an owned, mutable, dynamically-sized collection of UTF-8 characters. It's essentially a wrapper over a Vec<u8>, which means it behaves like a dynamic array that stores UTF-8 encoded characters. This allows for operations like appending or removing characters.

&str, often called a "string slice", is an immutable reference to a sequence of UTF-8 characters. It's essentially a pointer to the beginning of a string and its length. It doesn't own the data it points to, hence it's usually read-only. */

pub fn hash_maps() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}
