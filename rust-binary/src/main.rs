use std::cmp::Ordering;
// 通过 use 关键字引入 std 标准库中的 io 输入/输出库到当前作用域
use std::io;
// 下面这种方式可以简化导入，一次性导入标准库里的多个库
// use std::{cmp::Ordering, io};

// Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中
use rand::Rng;

// Rust 程序的入口函数，程序执行时会首先运行这个函数
fn main() {
    // println! 是 Rust 的一个宏，用于打印输出。当看到一个 !，则意味着调用的是宏而不是普通的函数
    println!("Guess the number!");

    let mut rng = rand::thread_rng();
    // 1..101 包含开始，不包含结尾；1..=100 包含开始和结尾
    let secret_number = rng.gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    // loop 关键字创建了一个无限循环
    loop {
        println!("Please input your guess.");

        // 在 Rust 中，变量默认是不可变的，在变量名前使用 mut 来使一个变量可变
        let mut guess = String::new();
        // let mut guess = String::from("我是默认值");

        // 如果程序的开头没有使用 use std::io; 引入 io 库，仍可以通过把函数调用写成 std::io::stdin 来使用函数
        // & 表示这个参数是一个引用，它允许多处代码访问同一处数据，而无需在内存中多次拷贝
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess == "quit\n" || guess == "exit\n" {
            // 主动退出程序
            std::process::exit(0);
        }

        // Rust 允许用一个新值来遮蔽 guess 之前的值。这允许复用 guess 变量的名字，而不是被迫创建两个不同变量，诸如 guess_str 和 guess 之类
        // 32 位数字 i32、32 位无符号数字 u32、64 位数字 i64，Rust 默认使用 i32，但这里显示指定了 u32，加上后续 guess 和 secret_number 做比较，Rust 会推断出 secret_number 也是 u32 类型
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // 将 expect 调用换成 match 语句，从而实现遇到错误就崩溃转换成处理错误
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please type a number! origin error: {}", err);
                // 意味着进入 loop 的下一次循环
                continue;
            }
        };
        println!("You guessed: {}", guess);

        let ordering = guess.cmp(&secret_number);
        match ordering {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
