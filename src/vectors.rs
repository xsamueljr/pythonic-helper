/// Return the highest number of a vector / array
pub fn max(vec: &[isize]) -> isize {
    let mut max = isize::MIN;

    for num in vec.iter() {
        if num > &max {
            max = *num
        }
    }

    max
}

/// Return the lowest number of a vector / array
pub fn min(vec: &[isize]) -> isize {
    if vec.is_empty() {
        return 0;
    }
    
    let mut min = isize::MAX;

    for num in vec.iter() {
        if num < &min {
            min = *num
        }
    }

    min
}


#[cfg(test)]
mod tests {
    use super::{max, min};

    #[test]
    fn test_max_and_min() {
        let vec: Vec<isize> = vec![20, 40, 1, 76, 240, 120];
        let highest_number = max(&vec);
        let lower_number = min(&vec);

        assert_eq!(240, highest_number);
        assert_eq!(1, lower_number);
    }
}