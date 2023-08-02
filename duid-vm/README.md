# Run  
## Read from cmd
cargo run -p duid-vm --bin repl --features interpreter
cargo run -p duid-vm --bin repl --features vm

## Run from files
cargo run -p duid-vm --bin duid_vm examples/simple.rd --features jit
cargo run -p duid-vm --bin duid_vm src/std/primitive.bid


# Steps  
## Step 1: Grammar-Lexer-Parser Pipeline 
0. Define the Grammar with Pest
1. Source Code
2. Tokenizer
3. Lexer/Lexical Analyzer
4. Parser

## Step 2: Build AST

## Step 3: Build Instructions from AST as ByteCode

