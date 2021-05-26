pub fn chunk<T>(target: &Vec<T>, size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let length = target.len();
    if size > length {
        panic!("size {} can't be bigger than length {}", size, length);
    }
    let mut result = Vec::<Vec<T>>::new();
    if length == 0 {
        return result;
    }
    let mut index = 0;

    while index < length {
        let end = index + size;
        let safe_end = if end > length { length } else { end };
        let slice = target[index..safe_end].to_vec();
        result.push(slice);
        index = end;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::chunk::chunk;

    #[test]
    fn chunk_works() {
        assert_eq!(
            chunk(&vec![3, 4, 2, 5, 7, 6], 2),
            vec![vec![3, 4], vec![2, 5], vec![7, 6]]
        );
    }
}
