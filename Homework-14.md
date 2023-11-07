## Secret Sharing
Try out Shamir secret sharing .    
- 1 - Create a polynomial with the secret being the constant term a0 , the other a values (a1. . . a4) can be chosen at random    
- 2 - The polynomial will be of the form    
>y(x) = a4x^4 + a3x^3 + a2x^2 + a1x + a0    
- 3 - Calculate the y values for five x values by evaluating the polynomial, these are the shares.  Reconstruct the polynomial using those shares and an online interpolation calculator such as   
		https://planetcalc.com/8680/
## Solution

> 413165x^4+624690x^3+337929x^2+750616x+42

Points: 
p(1),p(10),p(-1),p(7),p(-30) = {2126442, 4797639102, -624170, 1228090710, 318078637662}
Using the calculator, interpolates to:
![[Pasted image 20231106234433.png]]
