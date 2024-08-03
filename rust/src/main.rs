use std::ops::Add;

//use std::option;
//Option enum (some or none )
use evm::evm;//crate used to provide evm implementation
//to be added as dependency in Cargo.toml
use primitive_types::U256;//allows to use U256 type
use serde::Deserialize;

//The Debug trait is used to format a value using the {:?} formatter.
//The Deserialize trait is used to deserialize a data structure from a format like JSON, TOML, YAML, etc.
#[derive(Debug, Deserialize)]

//Seeing the format of the JSON file, we can see that it has a name, hint, code, and expect fields.

struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    expect: Expect,
    tx: Option<Tx>,
    state: Option<String>,
}


//The Code struct has two fields, asm and bin, which are strings.

#[derive(Debug, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
//The Expect struct has two fields: stack, success.
//The stack field is an optional vector of strings, and the success field is a boolean.
//Option : implies that the value is optional and can be None.
//enum Option<T> {
//    None,
//    Some(T),
//}

struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
    // #[serde(rename = "return")]
    // ret: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Tx{
    value : Option<String>,
    data: Option<String>,
    from : Option<String>,
    to : Option<String>,
    gas: Option<String>,
    origin: Option<String>,
    gasprice: Option<String>,
}

fn main() {
    let text = std::fs::read_to_string("../evm.json").unwrap();
    //The read_to_string function reads the contents of the file and returns a Result<String>.
    //The unwrap function is used to extract the value from the Result  or panic if it is an error.

    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();
    //serde_json deserealizes json string to text variable and then stored into vector where each element is of type struct Evmtest

    let total = data.len();
    //total should be equal to total  tests 
    //each test is a single element stored in vector as a struct 

        //data.iter() allows iterating over the vector data
        //enumerate adds index to each element
        //so tuple (index, test) is created for each element in vector data 
        //test is of type struct Evmtest
    for (index, test) in data.iter().enumerate() {

        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();
        //decodes the hex string (&str) into bytes and stores in code variable
        //code is a vector where each element is of type u8 (each opcode is a byte = 8 bits)
        let tx=&test.tx;
        let result = evm(&code, tx);
        //evm function declared as public in lib.rs file is called with code as argument
        //The evm function takes the bytecode (the reference to the code vector) and executes it in the EVM.
        //&code is the bytecode
      
        let mut expected_stack: Vec<U256> = Vec::new();
       //stack holds U256 values
    

        //the below line checks if expect.stack holds a value or not

        //if it does it is assigned to staacks then it pushes the value into expected_stack
        // If test.expect.stack is None, the code inside the block will be skipped.
          
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
            //str is converted to u256 and pushed into expected_stack
                //base 16 converted to u256

                //eg if test.expect.stack = ["0x01", "0x02", "0x03"]
                //then expected_stack = [1, 2, 3]
        }
        //The below code checks if the result of the test matches the expected result.
        //If it does not match, the test fails and the program panics.
        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }
        
        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            
            println!("\n");
            
            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
