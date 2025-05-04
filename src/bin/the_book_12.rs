//  The Book Section 12

// 目標: cargo run searchstring example-filename.txt のようにする。

// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     println!("{:?}", args);
// }

/**
 * ryokino@ryokino-FMVU95D2B:~/projects/recursion/backend_projecct_1/file_manipulator$ cargo run --bin the_book_12 needle haystack
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/the_book_12 needle haystack`
["target/debug/the_book_12", "needle", "haystack"]
 */

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let filename = &args[2];

//     println!("Searching for {}", query);
//     println!("In file {}", filename);
// }

/**
 * ryokino@ryokino-FMVU95D2B:~/projects/recursion/backend_projecct_1/file_manipulator$ cargo run --bin the_book_12 test sample.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/the_book_12 test sample.txt`
Searching for test
In file sample.txt
 */
use std::env;
use std::fs::File; // Fileを扱うのに必要
use std::io::prelude::*; // ファイル入出力を含む入出力処理をするのに有用なとレイトを含んでいる

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
