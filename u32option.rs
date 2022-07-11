
fn sum(list: &Vec<u32>) ->Option<u32>{
    let mut sum=0;
    for x in list
    {
        sum+= x;
    }
 // 结果 Result 被包装到 `Some` 取值中
   return Some(sum);
}

fn main() {
  let mut list: Vec<u32> = Vec::new();
  list.push(3u32);
  list.push(5u32);
  list.push(8u32);
  println!("{:?}", sum(&list));
}
