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


