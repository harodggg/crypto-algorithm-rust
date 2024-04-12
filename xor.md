# xor(exclusive Or)
```python3
0 ^ 0 = 0
1 ^ 1 = 0 

0 ^ 1 = 1
1 ^ 0  = 0

```
# Property
1. associative property
```python3
    a ^ b ^ c = (a ^ b) ^ c = a ^ (b ^ c)
```
2. Commutative property
```python3
    a ^ b ^ c = a ^ c ^ b
```
3. The operation between oneself and oneself always equals zero.
```python3
    a ^ a = 0 
    x ^ x = 0
```

4. The operation between itself and 0 always equals itself
```python3
    a ^ 0 = a
    x ^ 0 = x
```
# application
1. simplify calculations
```python3
 a ^ b ^ c ^ b ^ a = a ^ a ^ b ^ b ^ c = 0 ^ 0 ^ c = c
```

2. exchange value (the fastest method and no need space)
```python3 
x = a
y = b
x = x ^ y 
y = x ^ y = (x ^ y) ^y = x ^ 0 = x
x = x ^ y =(x ^ y) ^ x = (x ^ x) ^ y = 0 ^ y = y
```

3. encrypt

```python3
    key ^ Text = chiperText
    key ^ chiperText = Text
    (x ^ y) ^x = (x ^ x) ^ y = 0 ^ y = y
```

4. data backup
```python3 
    file x ,file y 
    x ^ y  = z 
    x is error 
    x = z ^ y = x ^ y ^ y = x

    y is error 
    y = z ^ x = x ^ y ^ x = y
```
