fn main() {
    let guess: u32 = "42".parse().expect("not a number");

    println!("{}", guess); 
    //rust一共四个标量类型：整数，浮点，布尔，字符(注意，emoji也可以是字符，属于unicode都行)
    //整数无小数部分，无符号u开头，有符号i，后面数字代表占用二进制数位数量
    //isize与usize位数由计算机架构决定，64位则64，默认类型i32
    //整数溢出：调试模式显示panic,发布模式环绕，即对u8模2^8，不显panic

    //浮点f32/64(单双精度)符合IEEE-754标准，默认类型为后者
    let x = 2.0;
    let y: f32 = 3.0; 
    //复合类型：元组（tuple），数组
    //tuple将多类型多指放于同一类型；长度固定，声明即不可改变
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    println!("{},{},{}", tup.0, tup.1, tup.2);
    //获取tuple值
    let (x, y, z) = tup;
    println!("{},{},{}", x, y,z);
    //数组也可以将多值放同类型，但各元素类型必须相同，长度亦固定
    //若数据想存于stack而非heap，或想保证有固定数量元素，用数组更好
    //还有vector存在，优于数组
    let a:[i32;5] = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov","Dec"];
    let first = months[0];
    let second = months[1];
    //数组在stack上分配的单个块的内存，可用索引访问，超过范围：编译通过，运行报错（rust不允许访问相应地址的内存）
    
    //声明函数用fn关键字，参数分形参与实参，函数签名中必须声明每个参数类型
    //函数体由语句组成，可选择由表达式结束，rust基于表达式，语句为执行动作的指令，表达式计算一个值
    //函数定义即语句，不返回值，故不可以用let将一个语句赋予变量
    another_function(s);
    //->符号后声明函数返回值类型但不可为其命名
    //rust中，返回值即函数体内最后一个表达式的值
    //若想提前返回，要用return，并指定值
    //有类似三目表达式写法
    //注意，if else分支返回类型必须是同一数据类型
    let num = if condition {0} else {1};
    //rust三种循环：loop while for
    //loop令rust反复执行某块代码直到喊停，用break来告知何时喊停
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter = 10{
            break counter* 2;
        }
    };
    println!("result is: {}", result);
    while number != 0 {
        println!("{}", number);

        number = number - 1;
    } 
    
}

fn another_function(x: i32) {
    println!("another one!value of x is {}", x);
}