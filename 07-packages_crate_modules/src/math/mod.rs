
// pub fn show_his_records( records:&mut Vec<String>) { //函数名称前增加pub关键字，该函数可被其他模块调用
//     println!("以下是您的猜数历史记录：");//模块中无print_info函数，修改为调用println!宏
//     for one_record in records {
//         println!("{}", one_record); //模块中无print_info函数，修改为调用println!宏。注意：参数不是字符串切片，需要增加占位符{}参数
//     }
// }

pub fn add(a:u32, b:u32) ->u32{
    a + b
}