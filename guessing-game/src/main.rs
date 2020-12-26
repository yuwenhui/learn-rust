use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");
  let secret_number = rand::thread_rng().gen_range(1..10);

  loop {
    println!("Please input your guess.");
  
    let mut guess = String::new();
    // 创建一个新的空字符串
  
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    // &mut guess 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。完成当前程序并不需要了解如此多细节。现在，我们只需知道它像变量一样，默认是不可变的。因此，需要写成 &mut guess 来使其可变，而不是 &guess
    // 如果程序的开头没有 use std::io 这一行，可以把函数调用写成 std::io::stdin()
    // read_line的返回值是io::Result类型，Result的类型是枚举，成员是Ok和Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且包含失败的前因后果。
  
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    // Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值
    // 这个功能常用在需要转换值类型之类的场景，它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量
    // 字符串的 parse 方法 将字符串解析成数字
    println!("You guessed: {}", guess);
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("To small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
