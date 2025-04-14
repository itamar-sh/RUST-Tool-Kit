fn main() {
    let vec1 = vec![5, 6, 2, 3, 9, 3];
    println!("{:?}", &basic_merge_sort(&vec1));

    let mut vec2 = vec![5, 6, 2, 3, 9, 3, 2, 1, 7, 6, 6, 4, 3, 2, 1];
    generic_in_place_merge_sort(&mut vec2);
    println!("\n{:?}", vec2);
}

fn basic_merge_sort(list: &Vec<i32>) -> Vec<i32> {
    if list.len() == 1 {
        return list.to_vec();
    }
    // Divide - New allocation each step
    let half1: Vec<i32> = list[..list.len() / 2].to_vec(); // Vec::from() requires the exact type to be known . to_vec() is not.
    let half2: Vec<i32> = list[list.len() / 2..].to_vec();
    // Sort
    let half1 = basic_merge_sort(&half1);
    let half2 = basic_merge_sort(&half2);
    // Conqure (Merge)
    basic_merge_two_sorted_lists(&half1, &half2)
}

fn basic_merge_two_sorted_lists(list1: &Vec<i32>, list2: &Vec<i32>) -> Vec<i32> {
    println!("sorting {:?} {:?}", list1, list2);
    let mut result = Vec::new();
    let mut p1 = 0;
    let mut p2 = 0;
    let len1 = list1.len();
    let len2 = list2.len();

    while p1 < len1 && p2 < len2 {
        if list1[p1] == list2[p2] {
            result.push(list1[p1]);
            result.push(list2[p2]);
            p1 += 1;
            p2 += 1;
        } else if list1[p1] > list2[p2] {
            result.push(list2[p2]);
            p2 += 1;
        } else {
            result.push(list1[p1]);
            p1 += 1;
        }
    }
    if p1 < len1 {
        result.extend(&list1[p1..]); // &&[i32] is a reference to iterable. Unlike &[i32]
    } else if p2 < len2 {
        result.extend(&list2[p2..]);
    }
    println!("sorting results {:?}", result);
    result
}

// Part 2 - Generic + in-place sorting
fn generic_in_place_merge_sort<T: Ord>(list: &mut Vec<T>) {
    generic_partiall_in_place_merge_sort(list, 0, list.len());
}

fn generic_partiall_in_place_merge_sort<T: Ord>(list: &mut Vec<T>, start: usize, end: usize) {
    if end - start <= 1 {
        return;
    }
    // Sort
    let half_index = start + (end - start) / 2;
    generic_partiall_in_place_merge_sort(list, start, half_index);
    generic_partiall_in_place_merge_sort(list, half_index, end);
    // Conqure (Merge)
    generic_in_place_merge_two_sorted_lists(list, start, half_index, half_index, end);
}

fn generic_in_place_merge_two_sorted_lists<T: Ord>(
    list: &mut Vec<T>,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
) {
    if list.len() == 1 {
        return;
    }
    // println!("sorting {:?}", list);
    let mut p1 = start1;
    while p1 < end1 {
        if list[p1] > list[start2] {
            list.swap(p1, start2);
            insert_first_element_to_sorted_vec(list, start2, end2);
        }
        p1 += 1;
    }
}

fn insert_first_element_to_sorted_vec<T: Ord>(
    list: &mut [T],
    element_index: usize,
    end_list_index: usize,
) {
    let mut p = element_index + 1;
    while p < end_list_index && list[p - 1] > list[p] {
        list.swap(p - 1, p);
        p += 1;
    }
}

/// tests suit
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut vec: Vec<i32> = vec![];
        generic_in_place_merge_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut vec = vec![42];
        generic_in_place_merge_sort(&mut vec);
        assert_eq!(vec, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5];
        generic_in_place_merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut vec = vec![5, 4, 3, 2, 1];
        generic_in_place_merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_vec() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
        generic_in_place_merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_strings() {
        let mut vec = vec!["banana", "apple", "cherry"];
        generic_in_place_merge_sort(&mut vec);
        assert_eq!(vec, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_duplicates() {
        let mut vec = vec![2, 3, 2, 3, 1];
        generic_in_place_merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 2, 3, 3]);
    }
}
