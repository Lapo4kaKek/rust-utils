use crate::parallel_compute;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_two() {
        let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected: Vec<Option<i32>> = vec![Some(3), Some(4), Some(5), Some(6), Some(7)];
        let func = |x: &i32| x + 2;
        assert_eq!(expected, parallel_compute::parallel_compute(vector, func));
    }
    
    #[test]
    fn multiply_two() {
        let vector: Vec<i32> = vec![-1, 7, 4, 11, 0, 1, 2, -7, 90, 100, -200, 56];
        let expected: Vec<Option<i32>> = vec![
            Some(-2),
            Some(14),
            Some(8),
            Some(22),
            Some(0),
            Some(2),
            Some(4),
            Some(-14),
            Some(180),
            Some(200),
            Some(-400),
            Some(112),
        ];
        let func = |x: &i32| x * 2;
        assert_eq!(expected, parallel_compute::parallel_compute(vector, func));
    }
    #[test]
    fn multiply_str() {
        let vector: Vec<String> = vec![
            String::from("Malasya"),
            String::from("Singapore"),
            String::from("China"),
            String::from("Italy"),
            String::from("Argentina"),
            String::from("Korea"),
            String::from("Spain"),
            String::from("Niger"),
            String::from("India"),
            String::from("Bachrein"),
            String::from("Canada"),
            String::from("Chili"),
        ];
        let expected: Vec<Option<String>> = vec![
            Some(String::from("I like Malasya")),
            Some(String::from("I like Singapore")),
            Some(String::from("I like China")),
            Some(String::from("I like Italy")),
            Some(String::from("I like Argentina")),
            Some(String::from("I like Korea")),
            Some(String::from("I like Spain")),
            Some(String::from("I like Niger")),
            Some(String::from("I like India")),
            Some(String::from("I like Bachrein")),
            Some(String::from("I like Canada")),
            Some(String::from("I like Chili")),
        ];
        let func = |text: &String| -> String { format!("I like {}", text) };
        assert_eq!(expected, parallel_compute::parallel_compute(vector, func));
    }
    #[test]
    fn test_empty_input() {
        let input: Vec<i32> = Vec::new();
        let result: Vec<Option<i32>> = parallel_compute::parallel_compute(input, |n: &i32| n * n);
        assert_eq!(result, Vec::<Option<i32>>::new());
    }
    
    #[test]
    fn test_small_input() {
        let input = vec![1, 2, 3, 4];
        let result = parallel_compute::parallel_compute(input.clone(), |n: &i32| n * n);
        let expected = input.into_iter().map(|n: i32| Some(n * n)).collect::<Vec<Option<i32>>>();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_large_input() {
        let input = (1..=20).collect::<Vec<i32>>();
        let result = parallel_compute::parallel_compute(input.clone(), |n: &i32| n * n);
        let expected = input.into_iter().map(|n: i32| Some(n * n)).collect::<Vec<Option<i32>>>();
        assert_eq!(result, expected);
    }
    
    fn concat(s: &str) -> String {
        format!("{}{}", s, s)
    }
    
    #[test]
    fn test_small_input_str() {
        let input = vec!["a", "b", "c", "d"];
        let input_str = input.iter().map(|c| c.to_string()).collect::<Vec<String>>();
        let result = parallel_compute::parallel_compute(input_str.clone(), |s| concat(&s));
        let expected = input_str.into_iter().map(|s| Some(concat(&s))).collect::<Vec<Option<String>>>();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_large_input_str() {
        let input = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
        let input_str = input.iter().map(|c| c.to_string()).collect::<Vec<String>>();
        let result = parallel_compute::parallel_compute(input_str.clone(), |s| concat(&s));
        let expected = input_str.into_iter().map(|s| Some(concat(&s))).collect::<Vec<Option<String>>>();
        assert_eq!(result, expected);
    }

    
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    impl Point {
        fn distance_from_origin(&self) -> Point {
            self.x.powi(2);
            return *self;
        }
    }
    
    #[test]
    fn test_large_input_struct() {
        let input_points = vec![
            Point { x: 1.0, y: 1.0 },
            Point { x: 2.0, y: 2.0 },
            Point { x: 3.0, y: 3.0 },
            Point { x: 4.0, y: 4.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 2.0, y: 6.0 },
            Point { x: 3.0, y: 7.0 },
            Point { x: 6.0, y: 8.0 },
            Point { x: 7.0, y: 1.0 },
            Point { x: 8.0, y: 2.0 },
            Point { x: 9.0, y: 3.0 },
            Point { x: 11.0, y: 4.0 },
            Point { x: 12.0, y: 7.4 },
            Point { x: 50.0, y: 3.3 },
            Point { x: 70.0, y: 4.5 },
            Point { x: 18.0, y: 776.0 },
            Point { x: 99.0, y: 777.0 },
            Point { x: 111.0, y: 66.0 },
            ];
            let result = parallel_compute::parallel_compute(input_points.clone(), 
            |p| p.distance_from_origin());
            let expected = input_points
            .into_iter()
            .map(|p| Some(p.distance_from_origin()))
            .collect::<Vec<Option<Point>>>();
            assert_eq!(result, expected);
    }
    #[test]
    fn test_parallel_compute_with_panic() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let results = parallel_compute::parallel_compute(input, |x| {
            if *x == 5 {
                panic!("Panic on value 5");
            }
            x * 2
        });

        assert_eq!(
            results,
            vec![
                Some(2),
                Some(4),
                Some(6),
                Some(8),
                None, // The result for x == 5 is None due to panic
                Some(12),
                Some(14),
                Some(16),
                Some(18),
                Some(20),
            ]
        );
    }
}