fn main() {
  let numbers_0: Vec<u32> = vec![];
  let numbers_4: Vec<u32> = vec![1, 2, 4, 5];
  let sum_0 = u32_sum(&numbers_0);
  let sum_4 = u32_sum(&numbers_4);
  assert_eq!(sum_0, None);
  assert_eq!(sum_4, Some(12));
}

fn u32_sum(numbers: &[u32]) -> Option<u32> {
  let res: u32 = numbers.iter().sum();
  if numbers.len() == 0 {
    None
  } else {
    Some(res)
  }
}
