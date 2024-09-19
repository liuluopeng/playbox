def partition_array(arr):
    def backtrack(start, current_partition):
        if start == len(arr):
            result.append(current_partition[:])
            return
        for i in range(start + 1, len(arr) + 1):
            new_partition = current_partition + [arr[start:i]]
            backtrack(i, new_partition)

    result = []
    backtrack(0, [])
    return result




arr = [1,2,3,4]
res = partition_array(arr)
for r in res:
    print(r)
    
    
# [[1], [2], [3], [4]]
# [[1], [2], [3, 4]]
# [[1], [2, 3], [4]]
# [[1], [2, 3, 4]]
# [[1, 2], [3], [4]]
# [[1, 2], [3, 4]]
# [[1, 2, 3], [4]]
# [[1, 2, 3, 4]]