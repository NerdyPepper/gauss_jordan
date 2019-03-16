# Gaussian Elimination (WIP)

algorithm to solve `A|B` systems using Gaussian Elimination, written in Rust.

### Input

```
01 02 01 -1 | -2
02 03 -1 02 | 07
01 01 03 -2 | -6
01 01 01 01 | 02
```

### Output

```
Echelon form:

01 02 01 -1 | -2
00 -1 -3 04 | 11
00 00 05 -5 | -15
00 00 00 01 | 02

--------------------
Gaussian eliminated:

01 00 00 00 | 01
00 -1 00 00 | 00
00 00 05 00 | -5
00 00 00 01 | 02

--------------------
X1 = 1
X2 = 0
X3 = -1
X4 = 2
```
