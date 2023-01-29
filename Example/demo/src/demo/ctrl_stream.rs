
#[allow(unused)]
mod demo1 {
    /// if
    pub fn main() {
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
}

#[allow(unused)]
mod demo2 {
    /// loop
    pub fn main() {
        let mut condition = 10;
        let loop_ret_value = loop {
            condition += 1;
            if condition == 20 {
                break condition;
            }
        };
        println!("loop return value is: {}", loop_ret_value); // loop return value is: 20
    }
}

#[allow(unused)]
mod demo3 {
    /// while
    pub fn main() {
        let mut condition = 3;
        while condition > 0 {
            println!("{}", condition);
            condition -= 1;
        }
        println!("leave while");
    }
}

#[allow(unused)]
mod demo4 {
    /// for
    pub fn main() {
        let ary = [1, 2, 3, 4, 5, 6];
        // 使用while循环遍历集合，需要先判断其范围，容易越界访问
        // let mut idx = 0;
        // while idx < 6 {
        //     println!("array[{}] value is: {}", idx, ary[idx]);
        //     idx-=1;
        // }

        // 使用for循环遍历集合，不需要当心越界访问问题，且效率更快
        // 缺点是无法得知当前的序号
        for ele in ary {
            // 这种方法是直接 array 中的值拷贝到 ele 变量中
            println!("array value is: {}", ele);
        }
        println!("=====================================");
        for ele in ary.iter() {
            // 这种方法是直接引用 array 中的值，并没有发生拷贝操作
            println!("array value is: {}", ele);
        }
    }
}

#[allow(unused)]
mod demo5 {
    /// 使用 range 库
    pub fn main() {
        for number in (1..4).rev() {
            println!("{}", number);
        }
        println!("LIFTOFF!");
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
}