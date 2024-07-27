use primitive_types::U256;
use serde::de::value;
pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

//function can accept any type that can be converted to a byte slice


pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc: usize = 0;

    let code = _code.as_ref();
    

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;

        if opcode == 0x00 {
            //println!("{} pc: ",pc);
            break;
        }
        else if opcode ==0x5f {
            let zero_u256: U256 = U256::from(0);
            stack.push(zero_u256);
            //println!("{} pc: ",pc);
            //println!(" stack: {}",stack[0]);
        }
        else if opcode == 0x60 {
            let value = U256::from(code[pc]);
            stack.insert(0,value);//inserts value at the beginning of the stack
            pc += 1;//increment the program counter
            //very important step 
            //not required if it is the only function 
            //but if several opcodes are there then it is required
            //since pc should point to the next opcode

        }
        else if opcode == 0x61{
            let value = (code[pc] as u16) << 8 | code[pc + 1] as u16;
            //<<8 shifts the bits of the first byte to the left by 8 bits
            //initial 8 bits stored as 16 bits and shifted to left
            stack.insert(0,U256::from(value));
            pc += 2;
        }
        else if opcode==0x63{
            let value = (code[pc] as u32) << 24 | (code[pc + 1] as u32) << 16 | (code[pc + 2] as u32) << 8 | code[pc + 3] as u32;
            stack.insert(0,U256::from(value));
            pc += 4;
        }
        else if opcode==0x65{
            let mut value = 0;
            for i in 0..6{
                //sort of recursion
                //value is shifted to left by 8 bits and then the next byte is added
                //then again the entire 16 bits is treated as value and then shifted to left by 8 bits
                //done 6 times
                value= value << 8 | code[pc + i] as u64; 
            }
            stack.push(U256::from(value));
            pc+=6;
        }
        else if opcode==0x69{
            let size= (opcode - 0x60+1) as usize;
            let mut value = 0;
            for i in 0..size{

                value= value << 8 | code[pc + i] as u128; 
            }
            stack.push(U256::from(value));
            pc+=size;
        }
        else if opcode==0x6a{
            let size= (opcode - 0x60+1) as usize;
            let mut value = 0;
            for i in 0..size{
                value= value << 8 | code[pc + i] as u128; 
            }
            stack.push(U256::from(value));
            pc+=size;
        }
        else if opcode==0x7f{
            let size=32;
            let mut bytes: [u8; 32] = [0; 32];//bytes is a fixed-size array of 32 bytes, initialized with zeros.
            //zeroes are of u8 types (similar to opcodes)
            //let mut bytes=U256::from(0);
            bytes.copy_from_slice(&code[pc..pc + size]);
            let value = U256::from(bytes);
            stack.insert(0,value);
            pc+=size;
        }
        else if opcode==0x50{
            stack.remove(0);
        }
        /*else if opcode==0x01{
            let mut v0=stack[0];
            let mut v1=stack[1];
            //TO prevent overflow
            if v0>U256::max_value()/2{
                v0=U256::max_value()-v0;
            }
            if v1>U256::max_value()/2{
                v1=U256::max_value()-v1;
            }

            if v0!=stack[0] || v1!=stack[1]{
                stack.remove(0);
                stack.remove(0);
                stack.insert(0,v0+v1-1);
            }
            else {
                stack.remove(0);
                stack.remove(0);
                stack.insert(0,v0+v1);
            }

        }*/
        else if opcode==0x01{
            let mut v0=stack[0];
            let mut v1=stack[1];
            stack.remove(0);
            stack.remove(0);
            stack.insert(0,add(v0,v1));
        }
        else if opcode==0x02{
            let mut v0=U256::from(stack[0]) ;
            let mut v1=U256::from(stack[1]);
            stack.remove(0);
            stack.remove(0);
            
            stack.insert(0,v0.overflowing_mul(v1).0   );
            //we can use overflowing_add in previous test suite

        }
      
        else if opcode==0x03{
            let mut v0=U256::from(stack[0]) ;
            let mut v1=U256::from(stack[1]);
            stack.remove(0);
            stack.remove(0);
           /*  if v0>=v1{
                stack.insert(0,v0-v1);
            }
            else{
                stack.insert(0,U256::max_value()-v1+v0+1);
            }*/
            stack.insert(0,subtract(v0,v1));

            //OR 
            //stack.insert(0,v0.overflowing_sub(v1).0   );
        }
        else if opcode ==0x04{
            let mut v0=U256::from(stack[0]) ;
            let mut v1=U256::from(stack[1]);
            stack.remove(0);
            stack.remove(0);
            stack.insert(0,divide(v0, v1));
        }
        else if opcode == 0x06{
            let mut v0=U256::from(stack[0]) ;
            let mut v1=U256::from(stack[1]);
            stack.remove(0);
            stack.remove(0);
            stack.insert(0,modulus(v0,v1));
            /* 
            if v1==U256::from(0){
                stack.insert(0,U256::from(0));
            }
            else{
                stack.insert(0, v0%v1)
            }*/
        } 
        else if opcode==0x08{
            let mut v0=stack[0];
            let mut v1=stack[1];
            let mut v2=stack[2];
            /* 
            //TO prevent overflow
            if v0>U256::max_value()/2{
                v0=U256::max_value()-v0;
            }
            if v1>U256::max_value()/2{
                v1=U256::max_value()-v1;
            }
            let mut vadd=U256::from(0);
            if v0!=stack[0] || v1!=stack[1]{
                stack.remove(0);
                stack.remove(0);
                vadd=v0+v1-1;

            }
            else {
                stack.remove(0);
                stack.remove(0);
                vadd=v0+v1;
            }
            stack.remove(0);
            stack.insert(0, vadd%v2) */
            stack.remove(0);
            stack.remove(0);

            stack.remove(0);

            let vadd=add(v0,v1);
            push(&mut stack, modulus(vadd, v2));

        }
        else if opcode == 0x09 {
            let mut v0=stack[0];
            let mut v1=stack[1];
            let mut v2=stack[2];

            stack.remove(0);
            stack.remove(0);
            stack.remove(0);

            v0=match v0.checked_rem(v2){
                Some(y)=>y,
                None=>U256::from(0),
            };
            v1=match v1.checked_rem(v2){
                Some(y)=>y,
                None=>U256::from(0),
            };
            let vmult=v0.overflowing_mul(v1).0;
            let vmod=match vmult.checked_rem(v2){
                Some(y)=>y,
                None=>U256::from(0),
            };
            stack.push(vmod);
        
        }

        else if opcode==0x0a{
            let mut v0=U256::from(stack[0]) ;
            let mut v1=U256::from(stack[1]);
            stack.remove(0);
            stack.remove(0);
            let exp=v0.pow(v1);
            stack.insert(0,exp);

        }
        else if opcode==0x0b{
            //Sign Extend : 
            //takes argument k and x  k is the byte position and x is the value to extend    
            //It extends the sign bit of x from the byte at position k. If k is 0, it means we are only considering the least significant byte of x.
            //If k is 31, it means we are considering the most significant byte of x.
            //If the MSB is 0, the number is positive.
            //If the MSB is 1, the number is negative.
            //If number of positive 

            let k = stack.remove(0); // Byte position (in U256)
            let x = stack.remove(0); // Value to be extended (in U256)
            let byte_pos = k.low_u32() as usize; 

            /* 
            //k converted to usize to use it as an index

            // Convert `x` to a byte array to extract the byte at the given position
                        //if we print x we will get 255

            let mut value_bytes = [0u8; 32];
            x.to_big_endian(&mut value_bytes);
        
            // Get the byte at the given byte position
            let sign_byte = value_bytes[31 - byte_pos]; //if we want 0th byte then 31-0=31
            let sign_bit = sign_byte & 0x80; // Extract the MSB of the byte at position `byte_pos`
            //In binary, 0x80 is 10000000, which means only the MSB is set (1), and all other bits are cleared (0).
            //& is the bitwise AND operator. It compares each bit of two numbers and returns a new number whose bits are set to 1 
            //only if the corresponding bits of both input numbers are also 1. Otherwise, the bit is set to 0.
            //Applying & 0x80 to sign_byte isolates the MSB of sign_byte.
            //Returns 0 if MSB is 0
            //U256 stores the most significant byte at the start (big-endian format).*/
            let extended_value = if sign(x,k) {
                // Extend sign bit for negative numbers
                /* 
                let bits_to_extend: usize = (byte_pos + 1) * 8;
                // The mask is created by shifting U256::MAX left by (byte_pos + 1) * 8 bits.
                //This covers all bits above the byte position with 1s.
                let mask = U256::MAX << bits_to_extend;
                x | mask*/
                extend(x,byte_pos)
            } else {
                // Positive numbers remain the same
                x
            };
        
            // Push the extended value back onto the stack
            stack.insert(0, extended_value);

         }
        else if opcode ==0x05{
            let mut v0=stack.remove(0);
            let mut v1=stack.remove(0);
            let v0_is_negative=is_negative(v0);
            let v1_is_negative=is_negative(v1);
            if is_negative(v0){
                v0=convert_twos_compliment(v0);
            }
            if is_negative(v1){
                v1=convert_twos_compliment(v1);
            }
            let mut div=divide(v0,v1);
            if v0_is_negative!=v1_is_negative{
                div=convert_twos_compliment(div);
            }
            //why is this required ???
            if v0==U256::from(0) || v1==U256::from(0){
                div=U256::from(0);
            }

            stack.insert(0, div);
            
            
        }
        else if opcode==0x07{
            let mut v0=stack.remove(0);
            let mut v1=stack.remove(0);
            let v0_is_negative=is_negative(v0);
            let v1_is_negative=is_negative(v1);
            if is_negative(v0){
                v0=convert_twos_compliment(v0);
            }
            if is_negative(v1){
                v1=convert_twos_compliment(v1);
            }
            let mut modul=modulus(v0,v1);
            if v0_is_negative && v1_is_negative{
                modul=convert_twos_compliment(modul);
            }

            stack.insert(0, modul);

        }
        else if opcode==0x10{
            let mut v0=stack.remove(0);
            let mut v1=stack.remove(0);
            let lt=less_than(v0,v1);
            if lt{
                stack.insert(0,U256::from(1));
            }
            else{
                stack.insert(0,U256::from(0));
            }
        }
        else if opcode==0x11{
            let mut v0=stack.remove(0);
            let mut v1=stack.remove(0);
            let lt=greater_than(v0,v1);
            if lt{
                stack.insert(0,U256::from(1));
            }
            else{
                stack.insert(0,U256::from(0));
            }
        }
        else if opcode==0x12{
            let mut v0=stack.remove(0);
            let mut v1=stack.remove(0);
            let v0_is_negative=is_negative(v0);
            let v1_is_negative=is_negative(v1);

            if is_negative(v0){
                v0=convert_twos_compliment(v0);
            }
            if is_negative(v1){
                v1=convert_twos_compliment(v1);
            }
            if v0==v1{
                stack.insert(0, U256::from(0));
                break;
            }
            let lt=less_than(v0,v1);
            if v0_is_negative!=v1_is_negative{
                if v0_is_negative==true{
                    stack.insert(0, U256::from(1));
                }
                else{
                    stack.insert(0, U256::from(0));
                }
            }
            if v0_is_negative==v1_is_negative{
                if v0_is_negative==true{
                    if !lt{
                        stack.insert(0, U256::from(0));
                    }
                    else{
                        stack.insert(0, U256::from(1));
                    }

                }
                else{
                    if lt{
                        stack.insert(0, U256::from(1));
                    }
                    else{
                        stack.insert(0, U256::from(0));
                    }
                }
            }
            
        }
        else if opcode==0x13{
            let mut v0=stack.remove(0);
            let mut v1=stack.remove(0);
            let v0_is_negative=is_negative(v0);
            let v1_is_negative=is_negative(v1);

            if is_negative(v0){
                v0=convert_twos_compliment(v0);
            }
            if is_negative(v1){
                v1=convert_twos_compliment(v1);
            }
            if v0==v1{
                stack.insert(0, U256::from(0));
                break;
            }
            let gt=greater_than(v0,v1);
            if v0_is_negative!=v1_is_negative{
                if v0_is_negative==true{
                    stack.insert(0, U256::from(0));
                }
                else{
                    stack.insert(0, U256::from(1));
                }
            }
            if v0_is_negative==v1_is_negative{
                if v0_is_negative==true{
                    if !gt{
                        stack.insert(0, U256::from(0));
                    }
                    else{
                        stack.insert(0, U256::from(1));
                    }

                }
                else{
                    if gt{
                        stack.insert(0, U256::from(1));
                    }
                    else{
                        stack.insert(0, U256::from(0));
                    }
                }
            }
            
        }
       

       
            

        



}  
    // TODO: Implement me

    return EvmResult {
        stack: stack,
        success: true,
    };
}

fn greater_than(x: U256, y: U256) -> bool {
    x > y
}
fn less_than(x: U256, y: U256) -> bool {
    x < y
}
fn convert_twos_compliment(x: U256) -> U256 {
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

fn is_negative(x:U256)->bool{
    x.bit(255)
}

fn sign(x:U256,k:U256)->bool{
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
fn extend(x:U256,bytes_pos:usize)->U256{
    let bits_to_extend: usize = (bytes_pos + 1) * 8;
    let mask = U256::MAX << bits_to_extend;
    x | mask
}
fn add (a:U256,b:U256)->U256{
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
fn modulus (c:U256,m:U256)->U256{
    if m==U256::from(0){
        return U256::from(0);
    }
    else{
        return c%m;
    }
}
fn multiply (a:U256,b:U256)->U256{
    return a.overflowing_mul(b).0;
}
fn subtract(a:U256,b:U256)->U256{
    if a>=b{
        return a-b;
    }
    else{
        return U256::max_value()-b+a+1;
    }
}
fn divide (a:U256,b:U256)->U256{
    if b==U256::from(0){
        return U256::from(0);
    }

    else{
        return a/b;
    }
}
fn push(stack: &mut Vec<U256>, value: U256) {
    stack.insert(0, value);
}

