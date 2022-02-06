// モジュールのモジュールなど階層化する場合はpubを頭につける（main.rsで呼び出す）
// pub mod sub_a;
// pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();
    // ミュータブルな変数の定義と型推論
    println!("ミュータブルな変数の定義と型推論");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // 使用していない定義には_をつけるとワーニングがなくなる
    let _i1 = 3;
    let _f1 = 0.1;
    println!("---------------------------------------------------------");
    // {:p}はメモリアドレスを取得する16進数なので注意
    // メモリアドレスは定義の前に＆をつける
    println!("使用しているPCのメモリサイズとconstについて");
    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);
    println!("---------------------------------------------------------");

    // ポインタはアドレスの番地でいいのかな？
    println!("ポインタについて");
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // シャドーイングとメモリの解放について
    println!("シャーディングとメモリの解放について");
    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);
    {
        let y = 0;
        println!("Stack address of y is: {:p}", &y);
        println!("The value of y is: {}", y);
    }
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    // タプル型について
    // 文字列と文字列スライスは別物
    println!("タプル型について");
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);
    println!("The value of t1 is: {} {} {}", _x, _y, _z);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    // 構造体などは{:?}という形で出力する
    println!("{:?}", t2);

    // 配列について
    println!("配列について");
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // 文字列スライス
    println!("文字列スライスについて");
    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";
    // これはメモリアドレス(先頭アドレス)（実データの保存情報のメモリアドレスなどを保存している領域）
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    // 静的記憶領域に紐づいているアドレスを取得する（実データ）
    println!("Static memory address of s1: {:?}", s1.as_ptr());
    println!("Static memory address of s2: {:?}", s2.as_ptr());
    // バイトサイズ
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());

    // 文字列
    println!("文字列について");
    let mut s1 = String::from("hello");
    let mut s2 = String::from("hello world");
    // メモリアドレス（データ格納情報）
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    // メモリアドレス（実データ）ヒープ領域の先頭アドレス
    println!("Heap memory address of s1: {:?}", s1.as_ptr());
    println!("Heap memory address of s2: {:?}", s2.as_ptr());
    // 文字列のバイト数
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    // ミュータブルのために確保している領域
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    // pushで文字列を追加する
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);
}
