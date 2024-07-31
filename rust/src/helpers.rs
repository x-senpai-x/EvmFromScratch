use primitive_types::U256;
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
