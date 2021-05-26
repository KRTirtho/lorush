pub fn base_difference<T>(
    collection: &Vec<T>,
    values: Vec<T>,
    iteratee: Option<fn(T) -> T>,
) -> Vec<T>
where
    T: Copy + PartialEq,
{
    let mut result = Vec::new();
    if collection.len() == 0 {
        return result;
    }

    'outer: for item in collection {
        let computed = match iteratee {
            Some(f) => f(*item),
            None => *item,
        };
        if values.iter().any(|val| *val == computed) {
            continue;
        }
        for value in &values {
            if *value == computed {
                continue 'outer;
            }
        }
        result.push(*item);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::difference::base_difference::base_difference;

    #[test]
    fn base_difference_works() {
        let res = base_difference(&vec![2, 1], vec![2, 3], None);
        assert_eq!(res, [1]);
    }
    #[test]
    fn with_iteratee() {
        let res = base_difference::<i32>(&vec![2, 1], vec![4, 3], Some(|v| v * v));
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn no_difference() {
        let res = base_difference::<i32>(&vec![2, 1], vec![2, 1], None);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn bigger_difference() {
        let res = base_difference::<i32>(
            &vec![2, 1, 5, 6, 9, 7, 6700, 244, 600, 1223, 800],
            vec![2, 3, 5, 6, 9, 7, 6700],
            None,
        );

        assert_eq!(res, vec![1, 244, 600, 1223, 800]);
    }
}
