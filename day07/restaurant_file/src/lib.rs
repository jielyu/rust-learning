mod front_of_house;

mod back_of_house;

// 测试use
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // use简写路径
    hosting::add_to_waitlist();

    //使用结构体
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("meal.toast={}", meal.toast);
    //meal.seasonal_fruit = String::from("blueberries"); // 编译报错

    // 使用枚举类型
    let order1 = back_of_house::Appetizer::Soup;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
