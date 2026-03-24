pub fn insertion_sort<T: PartialOrd>(a: &mut [T]) {
    let mut end = 0;
    while end < a.len() - 1 {
        for i in 0..=end {
            if a[end + 1] < a[i] {
                a[i..=(end + 1)].rotate_right(1);
                break;
            }
        }
        end += 1;
    }
}

#[cfg(test)]
macro_rules! test_any_sort {
    ( $f:ident ) => {
        let mut a = [45, 10, 34];
        $f(&mut a);
        assert_eq!(a, [10, 34, 45]);

        let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        $f(&mut a);
        assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut a = [1, 3, 2, 4, 6, 5, 8, 7, 9, 0];
        $f(&mut a);
        assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut a = [-5, 5, -3, 0, -1, 9, 42];
        $f(&mut a);
        assert_eq!(a, [-5, -3, -1, 0, 5, 9, 42]);

        let mut a = [
            std::f64::consts::PI,
            3.14,
            -6.18,
            0.,
            42.,
            -1.,
            f64::INFINITY,
            -f64::INFINITY,
        ];
        $f(&mut a);
        assert_eq!(
            a,
            [
                -f64::INFINITY,
                -6.18,
                -1.0,
                0.0,
                3.14,
                std::f64::consts::PI,
                42.0,
                f64::INFINITY
            ]
        );

        let mut a = [
            "1", "3", "2", "4", "6", "5", "8", "7", "AZ", "9", "0", "12", "az",
        ];
        $f(&mut a);
        assert_eq!(
            a,
            [
                "0", "1", "12", "2", "3", "4", "5", "6", "7", "8", "9", "AZ", "az"
            ]
        );

        let mut a: Vec<String> = a.into_iter().map(String::from).collect();
        $f(&mut a);
        assert_eq!(
            a,
            [
                "0", "1", "12", "2", "3", "4", "5", "6", "7", "8", "9", "AZ", "az"
            ]
        );
    };
}

#[test]
fn test_insertion_sort() {
    test_any_sort!(insertion_sort);
}
