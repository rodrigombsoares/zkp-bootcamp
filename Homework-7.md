Done on [/noir](noir/README.md)
## Noir
- Follow the instructions to install Rust and Noir   
- Create an example project    
- Create a new project   
	`nargo new hello_world`
- Build the project  
	`cd hello_world`    
	`nargo check`   
- Edit the inputs in the Prover.toml file  
	`x = "1" y = "2"`  
- Generate the proof  
	`nargo prove`  
- Verify the proof
	`nargo verify`  
	No output from the verifier signals success, if the proof fails to verify it will produce an error.