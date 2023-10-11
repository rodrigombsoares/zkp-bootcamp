Some modular arithmetic  
1. Working with the following set of Integers S={0,1,2,3,4,5,6}  
What is  
a) 4 + 4 = 1  
b) 3 x 5 = 1  
c) what is the inverse of 3?   
	Additive: 4, since 3+4 mod(7) = 0  
	Multiplicative: 5, since 3x5=1  

2. For S = {0,1,2,3,4,5,6}  
Can we consider 'S' and the operation '+' to be a group? Yes Z7  
This proves closure and inverses  

|   | 0| 1 | 2 | 3 | 4 | 5 | 6 |
| - | -- | -- |  -- | -- | -- | -- | -- |  
| 0 | 0| 1 | 2 | 3 | 4 | 5 | 6 |
| 1 | 1| 2 | 3 | 4 | 5 | 6 | 0 |
| 2 | 2| 3 | 4 | 5 | 6 | 0 | 1 |
| 3 | 3| 4 | 5 | 6 | 0 | 1 | 2 |
| 4 | 4| 5 | 6 | 0 | 1 | 2 | 3 |
| 5 | 5| 6 | 0 | 1 | 2 | 3 | 4 |
| 6 | 6| 0 | 1 | 2 | 3 | 4 | 5 |

3. What is: -13 mod 5?  
a=b mod(n) -> a-b=k * n  
x+13 = k * 5 -> x=2  

5. Polynomials  
For the polynomial x³ − x² + 4x − 12  
- Find a the positive root?   
	2 (x-2)(x²+1x+6)  
- What is the degree of this polynomial? 3  

In your teams discuss any systems you have used that involved zero knowledge proofs.   
Have you seen any applications of zero knowledge proofs other than with a blockchain?   
- ZCash (privacy), ZKRollups (scalability)  
What is to you, the most important feature of ZKP technology?   
- The main feature, being able to prove knowledge of a secret without revealing the secret   
Think of some use cases of zero knowledge proofs that you would like to see developed  
- Proof of identity (KYC) without hurting privacy  