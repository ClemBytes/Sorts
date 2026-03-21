pub fn insertion_sort(a: &mut [i64]) {
    let mut end = 0;
    while end < a.len() - 1 {
        let new_value = a[end + 1];
        for i in 0..=end {
            if new_value < a[i] {
                // Moves everything behind by one place
                for j in (i..=end).rev() {
                    a[j + 1] = a[j];
                }
                // Insert new value
                a[i] = new_value;
                break;
            }
        }
        end += 1;
    }
}

#[test]
fn test_insertion_sort() {
    let mut a = [45, 10, 34];
    insertion_sort(&mut a);
    assert_eq!(a, [10, 34, 45]);

    let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    insertion_sort(&mut a);
    assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut a = [1, 3, 2, 4, 6, 5, 8, 7, 9, 0];
    insertion_sort(&mut a);
    assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
