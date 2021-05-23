mod chunk;

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
