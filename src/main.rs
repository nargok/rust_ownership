fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    // 参照アドレスを渡そうとするとエラーになる
    // 消えるStringを渡そうとしているため
    // &s
    s
}
