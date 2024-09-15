pub fn binary_search(mut array: [u16; 8], target: u16) -> Option<String> {
    let mut start_index = 0;
    let mut end_index = array.len() - 1;
    let mut ok = Box::new("KOKOS");
    for i in 1..=8 {
        array[i - 1] = i as u16;
    }
    // assert_eq!(array[9], 10);

    while start_index <= end_index {
        // let middle_index = start_index + (end_index - start_index) / 2;
        let middle_index = (start_index + end_index) / 2;
        // assert_eq!(middle_index, 4);
        println!("Current middle index: {}", middle_index);

        if array[middle_index] == target {
            return Some(format!("Found at index: {}", middle_index));
        }
        if array[middle_index] < target {
            start_index = middle_index + 1;
        } else if array[middle_index] > target {
            end_index = middle_index - 1;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::binary_search;

    #[test]
    fn search_for_number_using_binary_search() {
        let empty_arr = [0; 8];
        // Vec::with_capacity()
        assert_eq!(empty_arr.len(), 8);
        let target = 1;
        let result = binary_search(empty_arr, target);
        if let Some(i) = result {
            assert_eq!(&i, "Found at index: 0");
        } else {
            panic!("No such value in array");
        }
    }
}
