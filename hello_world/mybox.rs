use std::ops::Deref;

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// 这实际上是一个tuple
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // https://doc.rust-lang.org/book/ch15-02-deref.html
    // 实际上在*和&符号中都调用了deref，把某个结构返回引用
    let x = 5;
    let y = MyBox::new(x);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
