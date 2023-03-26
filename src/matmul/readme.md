暴力求解
```
SQUARE-MARTIX-MULTIPLAY(A,B)
n = A.rows
let C be a new n*n matrix
for i = 1 to n 
    for j = 1 to n
        c[i][j] = 0
        for k = 1 to n
            c[i][j] = c[i][j]+a[i][k]*b[k][j]
return C
```
