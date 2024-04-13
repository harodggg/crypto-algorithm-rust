# Implementation process
1. KEY_INIT,KEY_BYTES, 64 bytes = 16 * 4 bytes = 16 * 32 bit
2. 
```
KEY_INIT[u8;64] = {
    KEY_INIT[0] ~ KEY_INIT[15]: constant,16 bytes,
    KEY_INIT[16] ~ KEY_INIT[47]: key,47 -15=32bytes, 
    KEY_INIT[48] ~ KEY_INIT[51]: block counter, 4bytes,
    KEY_INIT[52] ~ KEY_INIT[63]: nonce, 12 btyes
}
```
```
cccccccc  cccccccc  cccccccc  cccccccc
kkkkkkkk  kkkkkkkk  kkkkkkkk  kkkkkkkk
kkkkkkkk  kkkkkkkk  kkkkkkkk  kkkkkkkk
bbbbbbbb  nnnnnnnn  nnnnnnnn  nnnnnnnn
```
3.  4 * 4 matrix
4.  A round is a combination of steps. the "20" of chacha20 means having 20 rounds
5. quarter round.A round has four quarter round. chacha20 has 20 quarter round
6. quarter round means:
```
a += b; d ^= a; d <<<= 16;
c += d; b ^= c; b <<<= 12;
c += b; d ^= a; d <<<= 8;
c += d; b ^= c; b <<<= 7;
```
7. column operations
| :--: | :--: | :--: | :--: |
0 1 2 3 
4 5 6 7 
8 9 10 11 
12 13 14 15 

the first quarter round: 0 4 8 12 
the second quarter round: 1 5 6 7 
the third quarter round: 2 6 10 14
the fourth quarter round: 3 7 11 15

8. diagonal operations
| :--: | :--: | :--: | :--: |
0 1 2 3 
4 5 6 7 
8 9 10 11 
12 13 14 15 

the first quarter round: 0 5 10 15 
the second quarter round: 1 6 11 12
the third quarter round: 2 7 8 13
the fourth quarter round: 3 4 9 14

9. chacha 20 = 10 * (cloum operations + diagonal operations) = 10 * ( 4 * cloum quarter round + 4 * diagonal quarter round)
10. matrix M, 20 ROUND M, get new matrix S. M + S =new matrix W
11. Use little endian to organize W= 64 bytes.
12. 64 bytes ^ text = 64 key
13. The data is divided into several 64bytes blocks. 64 bytes_block ^ 64 bytes key_n . block counter + 1  after processing every block
14. key_init  16 bytes contants = "expand 32-byte k"
