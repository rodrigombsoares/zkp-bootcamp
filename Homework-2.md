1. **Modular arithmetic - you just need to find examples, you don't need to prove anything.**
	- **a) Is it true that all odd squares are ≡ 1 (mod 8)?**
		Proof 1
		```
		an odd number is of the form 2k+1
		(2k+1)^2 = 4k^2+4k+1 = 4k(k+1)+1
		4k(k+1) is always divisible by 8 (either k or k+1 is pair)
		thus 1 is the remainder
		```
		Proof 2
		```
		S = {0,1,2,3,4,5,6,7} (integers mod 8) is the only set we need, since every Z mod 8 falls into this set (eg.: 9≡1mod(8))
		So, every odd number in Z is one of {1,3,5,7} (mod 8) since
		o≡b mod 8 -> o-b=8*k, o is odd, 8*k is even, b must be odd
		Now we can prove by testing all:
			1^2≡1, 3^2=9≡1, 5^2=25≡1, 7^2=49≡1 (mod 8)
		```
	- **b) what about even squares (mod 8)?**  
		Besides 2, every even number squares are ≡0 (mod 8)
2. **Try out the vanity bitcoin address example at asecurity or the [Ethereum](https://vanity-eth.tk/) version.** 
3. **What do you understand by** 
	- **O(n)**
	- **O(1)**
	- **O(log n)**
		Big O notations defines asymptote for functions, characterizes functions according to their growth rates. O(n)>O(log n)>O(1)  
	**For a proof size, which of these would you want?**  
		The best size of a proof would be O(1).