fn main() {
  // 定义s1并分配存储
  let s1 = String::from("Hello");

  // 克隆s1至s2，s1仍然有效
  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);

  let x = vec![1, 2];
  let y = x.clone();
  println!("x: {:?}", x);
  println!("y: {:?}", y);
}

// s1，s2作用域结束，分别释放存储
