//2022-07-11 by ming

fn sum(list: &Vec<u32>) ->Option<u32>{
// 定义 Result 为 `Some（0）`
    let mut result:Option<u32> = Some(0);
    for x in list
    {
        result = x.checked_add(result.unwrap());
    }
    if result.is_none() {
        return None;
    }
 
    return result;
}

fn main() {
  let mut list: Vec<u32> = Vec::new();
  list.push(3u32);
  list.push(5u32);
  list.push(8u32);
  println!("{:?}", sum(&list));
}
