## RiscZero 
### Prerequisites 
You need to have rustup installed. 
### Install cargo-risczero 
		cargo install cargo-risczero
		cargo risczero install
### Create a starter project 
		cargo risczero new homework-risc-zero 
### Understand the template. 
Follow [Understanding the RISC Zero zkVM Starter Template](https://www.risczero.com/docs/examples/understanding_template)

## Solution
- First, got a deeper understanding of RISC Zero following [this playlist](https://www.youtube.com/watch?v=cLqFvhmXiD0&list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5&index=2)  
- The [5th video](https://youtu.be/Yg_BGqj_6lg?si=PJViQ4735iDnRJEN) has an awesome explanation on how to use the template  

> ⚠️ This build will take a while (20+ min), so hold on.
> `Building [=======================> ] 278/284: risc0-circuit-rv32im-sys(build)`

>The API for the receipt has changed a little bit. You access journal and deserialize:
	`let digest: Digest = from_slice(&receipt.journal).unwrap();`

- Check online to see if the printed sha is correct. And it is :) 
- 