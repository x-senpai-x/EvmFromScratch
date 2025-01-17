use primitive_types::U256;
pub fn remove_one(stack: &mut Vec<U256>) -> U256 {
    stack.remove(0)
}

pub fn remove_two(stack: &mut Vec<U256>) -> (U256, U256) {
    let a = stack.remove(0);

    let b = stack.remove(0);
    (a, b)
}
pub fn remove_three(stack: &mut Vec<U256>) -> (U256, U256, U256) {
    let a = stack.remove(0);
    let b = stack.remove(0);
    let c = stack.remove(0);
    (a, b, c)
}
pub fn push(stack: &mut Vec<U256>, value: U256) {
    stack.insert(0, value);
}

pub fn add (a:U256,b:U256)->U256{
    let mut v0=a;
    let mut v1=b;
    if v0>U256::max_value()/2{
        v0=U256::max_value()-v0;
    }
    if v1>U256::max_value()/2{
        v1=U256::max_value()-v1;
    }

    if v0!=a || v1!=b{
        return v0+v1-1;
   
        
    }
    else {
       
        return v0+v1;
    }
}
pub fn multiply(mut a:U256,mut b:U256)->U256{
    a.overflowing_mul(b).0
}
pub fn subtract(a:U256,b:U256)->U256{
    if a>=b{
        return a-b;
    }
    else{
        return U256::max_value()-b+a+1;
    }
}
pub fn divide (a:U256,b:U256)->U256{
    if b==U256::from(0){
        return U256::from(0);
    }

    else{
        return a/b;
    }
}
pub fn modulus (c:U256,m:U256)->U256{
    if m==U256::from(0){
        return U256::from(0);
    }
    else{
        return c%m;
    }
}
pub fn dup_n(stack: &mut Vec<U256>,n:usize)->U256{
    stack[n-1]
}
pub fn swap(stack: &mut Vec<U256>,n:usize){
    let temp=stack[0];
    stack[0]=stack[n];
    stack[n]=temp;
}
pub fn shr(mut shift: U256, mut num: U256) -> U256 {

    let shift_amount: usize = shift.low_u64() as usize;
    num = num >> shift_amount;
    num
}
pub fn equalto(x: U256, y: U256) -> bool {
    x == y
}
pub fn greater_than(x: U256, y: U256) -> bool {
    x > y
}
pub fn less_than(x: U256, y: U256) -> bool {
    x < y
}


pub fn convert_twos_compliment(x: U256) -> U256 {
    // Note, normally the twos compliment of 0 is 0
    // However according to the EVM spec it seems to want this behaviour
    // I am uncertain if this is a misunderstanding by me/edge case for the SAR opcode
    // TODO: research this behaviour
    if x == U256::zero() {
        return !x;
    }
    // We do this by first doing a bitwise negation then adding one
    !x + U256::one()
}

pub fn is_negative(x:U256)->bool{
    x.bit(255)
}

pub fn sign(x:U256,k:U256)->bool{
    let mut value_bytes = [0u8; 32];
    let mut bytes_pos = k.low_u32() as usize;
    x.to_big_endian(&mut value_bytes);
    let sign_byte = value_bytes[31-bytes_pos];
    let sign_bit = sign_byte & 0x80;
    if sign_bit!=0{
        return true;//positive
    }
    else {
        return false;
    }
}
pub fn extend(x:U256,bytes_pos:usize)->U256{
    let bits_to_extend: usize = (bytes_pos + 1) * 8;
    let mask = U256::MAX << bits_to_extend;
    x | mask
}
