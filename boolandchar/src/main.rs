fn main() {
    // 使用bool 關鍵字 取值為 true 或 false
    let checked: bool = true;

    println!(
        "{}",
        if checked {
            "checked is true"
        } else {
            "checked is false"
        }
    );

    // 字符: 是字符串的基本組成 也就是單個字符或字 UTF-8作為底層編碼,包含數字 字母 Unicode 和 其他特殊字符
    let c:char = 'R';

    println!("{}", c);
}