### MAX_HEAPIFY 维护堆的性质

```
MAX-HEAPIFY(A,i)
    l = LEFT(i)
    r = RIGHT(i)
    if l<=A.heap-size and A[l]>A[i]
        largest = l
    else largest = i
    if r<=A.heap-size and a[r]>A[largest]
        largest =r
    if largest!=i
        exchange A[i] with A[largest]
        Max-HEAPIFY(A,largest)
```

### BUILD_MAX_HEAP 从无序的输入数组中构造一个最大堆

渐进紧确复杂度为 O(n)

```
BUILD-MAX-HEAP(A)
    A.heap-size = A.length
    for i = A.length/2 downto 1
        MAX-HEAPIFY(A,i)
```

### HEAPSORT O(nlgn)对数组进行原址排序

```
HEAPSORT(A)
    BUILD-MAX-HEAP(A)
    for i = A.lenth downto 2
        exchange A[1] with A[i]
        A.heap-size = A.heap-size -1
        MAX-HEAPIFY(A,1)
```

### 优先队列

快排的性能优于堆排序，但堆数据结构仍然有很多应用，比如：作为高效的优先队列
