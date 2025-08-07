// 定义TokenInstruction 结构体
// 需要实现编译和反序列化
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TokenInstruction {
    // 创建Token
    CreateToken { decimals: u8 },
    // 铸币
    Mint { amount: u64 },
}
