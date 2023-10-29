## Using Circom REPL 
See this partial circom [code](https://gist.github.com/extropyCoder/fe99bf4b0094354edaf7b737b14ffa0f)   
Using this code in the [zkREPL](https://zkrepl.dev/), complete the constraint on line 19 and add some appropriate inputs in the input section.   
Test that it creates a proof, and show that an incorrect proof fails.
## Team Discussion 
Imagine you are developing a project and want to use one of the zkRollup based L2s. 
1. What factors would be important to you when choosing which to use. 
2. Of the protocols we have seen so far, which would you choose? 
3. The Mina - ETH bridge uses a STARK to prove the verification of a SNARK proof, what could be the rational behind mixing these 2 types of proving systems?
## Solutions
``` circom
pragma circom 2.0.0;

/*This circuit template checks that c is the multiplication of two inputs */
template Multiplier2 () {
	// Declaration of signals.
	signal input in1;
	signal input in2;
	signal output c;

// Constraints.
	c <== in1 * in2;
}

component main {public [in1,in2]} = Multiplier2();

/* INPUT = {
	"in1": "5",
	"in2": "77"
} */
```
