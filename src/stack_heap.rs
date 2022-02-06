enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // スタックの中に持てる領域は8MBが上限
    // let a1: [u8; 9000000] = [1; 9000000];
    // 実行時に配列の要素を変更するときはvec!ベクター型を使用する
    // ベクター型は24bytes
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    // Heap内の実データの先頭アドレス
    println!("Heap memory address of v1: {:?}", v1.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    // 要素をついか（先頭に追加されます）
    v1.insert(1, 10);
    println!("{:?}", v1);
    // 要素の削除
    v1.remove(0);
    println!("{:?}", v1);
    // 配列の連結
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);

    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of tuple data is: {:p}", &t1);
    println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr());
    println!("Len of t1.1 is: {}", t1.1.len());
    println!("Capacity of t1.1 is: {}", t1.1.capacity());
    let mut b1 = Box::new(t1);
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);
    println!("Stack address of box pointer is: {:p}", &b1);
    println!("Heap address of tuple data is: {:p}", b1);
}
