## Practice using Zokrates 

- Use [Remix](remix.ethereum.org) 
- Install the Zokrates plugin 

1. **Use the example file to generate a proof to show that a prover knows the square root of 25.**   
```python
def main(private field a, field b) {
	assert(a * a == b);
	return a * a == b;
}
```
Compile > Compute a=5 b=25 > Setup > Generate Proof > Export Verifier  
Go to the `verifier.sol` and deploy on remixVM.
![[Pasted image 20231013180016.png | 200]]
Get `proof` and `input` from generate proof tab.

2. **Try to create an invalid proof.**   
Go on compute, add a=4 b=25 for example. It will raise an error before you're able to setup.
You can also input a wrong value like `0x0000000000000000000000000000000000000000000000000000000000000010` in the compiled contract and the transaction is going to return false.   

3. Follow the example to build a proof that you know the pre image of a hash https://zokrates.github.io/examples/sha256example.html
Do the same thing as above

4. In principle how could you use Zokrates to verify that a certain address on Ethereum has more than say 1 ETH?