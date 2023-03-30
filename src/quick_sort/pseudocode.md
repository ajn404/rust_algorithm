## 快排

```
QUICKSORT(A,p,r)
    if p<r
        q = PARTITION(A,p,r)
        QUICKSORT(A,p,q-1)
        QUICKSORT(A,q+1,r)
```

```
PARTITION(A,p,r)
    x =A(r)
    i = p-1
    for j = p  to r-1
        if(A[j]<=x)
            i++
            exchange A[i] with A[j]
    exchange A[r] with A[i+1]
    return i+1
```

快速排序算法的期望运行时间为 O(nlgn)
