struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    // シングルクォーテーションの場合はchar型になる（４バイト）
    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", largest(char_list));
    println!("{}", largest(number_list));
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 1.0 };
    let p3 = PointAnother { x: 5, y: 10.4 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("{} {}", p1.x, p1.y);
    println!("{} {}", p2.x, p2.y);
    println!("{} {}", p5.x, p5.y);

}

// 配列の中から最大値を取得する
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
