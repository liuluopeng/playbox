n = int(input())
a = list(map(int, input().split()))

# 对数组进行排序
a.sort()

# 输出排序后的结果
for num in a:
    print(num, end=' ')