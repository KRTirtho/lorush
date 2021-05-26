mod base_difference;
use base_difference::base_difference;

pub fn difference<T>(collection: &Vec<T>, values: &Vec<Vec<T>>) -> Vec<T>
where
    T: Copy + PartialEq,
{
    let mut res = Vec::new();
    for value in values {
        for values_value in value {
            res.push(*values_value);
        }
    }
    base_difference(collection, res, None)
}

#[cfg(test)]
mod tests {
    use crate::difference::difference;

    #[test]
    fn difference_works() {
        let res = difference(&vec![2, 1], &vec![vec![2, 3]]);
        assert_eq!(res, vec![1]);
    }
    #[test]
    fn no_difference() {
        let res = difference::<i32>(&vec![2, 1], &vec![vec![2, 1]]);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn bigger_difference() {
        let res = difference::<i32>(
            &vec![2, 1, 5, 6, 9, 7, 6700, 244, 600, 1223, 800],
            &vec![vec![2, 3, 5], vec![6, 9, 7, 6700]],
        );

        assert_eq!(res, vec![1, 244, 600, 1223, 800]);
    }
}
