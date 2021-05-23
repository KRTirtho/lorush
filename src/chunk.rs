pub fn chunk<T>(target: &Vec<T>, size: usize)->Vec<Vec<T>> where T: Clone{
  let length = target.len();
  if size>length{
    panic!("size {} can't be bigger than length {}", size, length);
  }
  let mut result = Vec::<Vec<T>>::new();
  if length==0{
    return result;
  }
  let mut index = 0;
  
  while index < length {
    let end = index+size;
    let safe_end = if end>length {length} else {end};
    let slice = target[index..safe_end].to_vec();
    result.push(slice);
    index = end;
  }
  
  result
}