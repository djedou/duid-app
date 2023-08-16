# Run  
## Read from cmd
cargo run -p binuid --bin repl --features interpreter
cargo run -p binuid --bin repl --features vm

## Run from files
cargo run -p binuid --bin binuid examples/simple.rd --features jit
cargo run -p binuid --bin binuid src/std/primitive.bid


# Steps  
## Step 1: Grammar-Lexer-Parser Pipeline 
0. Define the Grammar with Pest
1. Source Code
2. Tokenizer
3. Lexer/Lexical Analyzer
4. Parser

## Step 2: Build AST

## Step 3: Build Instructions from AST as ByteCode

