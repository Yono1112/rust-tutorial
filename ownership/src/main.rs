fn main() {
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}, world!", s1); // s1はs2にムーブされているためエラー
        println!("s2: {}, world!", s2);
    }

    {
        let s3 = String::from("hello");
        let s4 = s3.clone();
        println!("s3: {}, world!", s3); // s3はs4にクローンされているためエラーにならない
        println!("s4: {}, world!", s4);
    }

    {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y); // 整数型はスタックに保存されるためムーブではなくコピーされる
    }


    let s = String::from("hello");  // sがスコープに入る
    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない
    // println!("{}", s);           // sはムーブされているためエラー

    let x = 5;                      // xがスコープに入る
    makes_copy(x);                  // xも関数にムーブされるが、i32はCopyなので、この後にxを使っても大丈夫
    println!("{}", x);               // xはi32なのでCopyされるためエラーにならない
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
