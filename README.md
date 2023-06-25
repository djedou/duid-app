# duid-app
**duid-app**: **D**jedou **UI** **D**esign **App**lication  

# Working by cloning the repository.  
## Installation  
```  
cd cargo-duid-app  
cargo install --path .
```
  
## Create project  
```  
cargo-duid-app init --name <PROJECT_NAME>  
```  
  
## Develop your project  
```  
cd <PROJECT_NAME>  
cargo-duid-app dev  
```  

## Run develop server 
From another terminal      
```  
cargo-duid-app serve // default --host="0.0.0.0" --port=3000 
```
or  
```  
cargo-duid-app serve --host <HOST>  --port <PORT>  
```  

## Build your project    
```  
cargo-duid-app build  
``` 

## Run production server  
```  
cargo-duid-app deploy // default --host="0.0.0.0" --port=3000  
```  
or  
```
cargo-duid-app deploy --host <HOST>  --port <PORT>  
``` 

# Working from crates.io  
## Install 
NB: Not yet on crates.io.  
```  
cargo install cargo-duid-app  
```  
  
## Create project  
```  
cargo duid-app init --name <PROJECT_NAME>  
```  
  
## Develop your project  
```  
cd <PROJECT_NAME>  
cargo duid-app dev  
```  

## Run develop server 
From another terminal      
```  
cargo duid-app serve // default --host="0.0.0.0" --port=3000  
```  
or  
```
cargo duid-app serve --host <HOST>  --port <PORT>  
```  

## Build your project    
```  
cargo duid-app build  
``` 

## Run production server  
```  
cargo duid-app deploy // default --host="0.0.0.0" --port=3000  
```  
or  
```
cargo duid-app deploy --host <HOST>  --port <PORT>  
``` 

# Structure:
The folder **./src/app** should have a file called **page.rs** which represents **/** the **root path** of your application.  
Each folder in the directory **./src/app** represents a **path**.      
Each folder in the directory **./src/app** should have the file called **page.rs**.  
Each **page.rs** should look like this format  
```
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn duid() {
    // This is the entry point of this page.
    // your code goes in this function
}
```