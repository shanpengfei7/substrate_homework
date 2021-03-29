fn main() {
    let args = &vec![127, 200, 4];

    // 递归方式
    match sum_by_recursive(&args) {
        Some(result) => println!("sum by recursive is {}", result),
        None => println!("none"),
    }
    println!("===================");
    // fold方式(先实现的递归，但是感觉递归代码太繁琐了，搞一个内置的遍历方法吧)
    match sum_by_fold(&args) {
        Some(result) => println!("sum by fold is {}", result),
        None => println!("none"),
    }
}

fn sum_by_fold(args: &[u32]) -> Option<u32> {
    let result = args.iter().fold(Some(0u32), |sum, x| match sum {
        Some(result) => result.checked_add(*x),
        None => {
            println!("number overflow");
            None
        }
    });

    result
}

fn sum_by_recursive(args: &[u32]) -> Option<u32> {
    recursive(args, 0)
}

fn recursive(args: &[u32], index: usize) -> Option<u32> {
    // 从列表中取元素
    match args.get(index) {
        // 当取当元素时，正常操作
        Some(num) => {
            // 递归调用，继续求和
            let result = recursive(args, index + 1)?;
            // 求和并返回，checked_add，溢出时返回None
            match result.checked_add(*num) {
                Some(r) => Some(r),
                None => {
                    println!("number overflow");
                    None
                }
            }
        }
        // 当取不到元素时，返回0
        None => {
            println!("get the last element");
            Some(0)
        }
    }
}
