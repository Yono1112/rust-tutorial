fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // s1への参照を渡す
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2: '{}'", s2);


    let r1 = &mut s2;
    // let r2 = &mut s; // ここでエラーが発生する
    // println!("{}, {}", r1, r2) // 特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てない
    println!("r1: {}", r1);

    bad_reference();
}

fn calculate_length(s: &String) -> usize { // sはStringの借用(関数の引数に参照を取ること)を受け取る
    s.len()
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので、何も起こらない

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("changed string: '{}'", some_string);
}

fn bad_reference() {
    // let reference_to_nothing = dangle(); // エラーが発生する
    let reference_to_anything = no_dangle(); // エラーが発生しない
    println!("reference_to_anything : {}", reference_to_anything);
}

// fn dangle() -> &String { // Stringへの参照を返す
//     let s = String::from("hello");
//     &s
// } // ここでsがスコープを抜けるため、参照先が無効になるため、エラーが発生する

fn no_dangle() -> String { // Stringを返す
    let s = String::from("hello");
    s
} // ここでsがスコープを抜けるが、所有権が移動しているため、何も起こらない