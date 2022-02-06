// モジュールのモジュールなど階層化する場合はpubを頭につける（main.rsで呼び出す）
pub mod sub_a;
pub mod sub_b;

pub fn run () {
  println!("Here is vars module!!");
  sub_a::func_a();
  sub_b::func_b();
}