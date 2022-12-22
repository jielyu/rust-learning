pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    //     assert_ne!(result, 3);
    //     // 测试失败的情况
    //     assert!(result != 4, "failed to calculate {}", result);
    // }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("test panic");
    }

    #[test]
    #[should_panic(expected = "test")]
    fn test_panic2() {
        panic!("test panic");
    }

    // #[test]
    // fn test_result() -> Result<(), String> {
    //     if 2 + 3 == 4 {
    //         Ok(())
    //     } else {
    //         Err(String::from("check calculation failed"))
    //     }
    // }

    #[test]
    #[ignore]
    fn test_ignore() {
        assert!(2 + 3 == 5)
    }
}
