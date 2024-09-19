# python
需要了解的: 

class method 
frozen 
typing Self
... 



## Poetry 工具
2024年7月5日00:18:23

Python依赖管理及打包利器Poetry常用命令大全（非常详细）零基础入门到精通，收藏这一篇就够了_python poetry-CSDN博客 
https://blog.csdn.net/Javachichi/article/details/138857884 

命令:
```


poetry new project_name 
或者已经有的项目: 
poetry init 

poetry install 

进入虚拟环境:
poetry shell 

添加第三方库:
poetry add numpy 

```



## todo
- [ ] **解包



# 写法

### 检测本段代码是否可作为main function执行：
  ```python 
    if __name__ == '__main__':
        pass 
  ```

### 使用dict()对数组计数
```python
mylist = [1,1,2,3,4]

mydict = {}
for num in mylist:
    if num in mydict:
        mydict[num] += 1
    else:
        mydict[num] = 1 
```



# 注意的地方
### 交换元素并不是同时交换的

剑指offer 查找重复的元素

```python

# not ok
nums[i], nums[nums[i]] = nums[nums[i]], nums[i]
nums[i], nums[nums[i]] = nums[nums[i]], nums[i] 时， 
第一步：先改变nums[i]=nums[nums[i]]，
第二步：再改变 nums[nums[i]]=nums[i]时， 
由于nums[i]的值已经发生变化， 
即此时的nums[nums[i]]已不等于第一步里的 nums[nums[i]]。 
即: a, b = b,a 变成了a = b, c = a
# ok 
nums[nums[i]], nums[i] = nums[i], nums[nums[i]]

# Python交换元素，看似同时赋值，然而不是同时的。。。
# 为了避免烦了，另外用两个变量保存住nums[i]

class Solution:
    def findRepeatNumber(self, nums: List[int]) -> int:
        for i in range(len(nums)):
            while nums[i] != i:
                if nums[i]==nums[nums[i]]:
                    return nums[i]
                else:
                    indexLeft = i 
                    indexRight = nums[i]
                    nums[indexLeft], nums[indexRight] = nums[indexRight], nums[indexLeft]

```



### 降序的range()函数仍应该注意左闭右开

在Python，递减从len(nums)到0, 应该是range(len(nums),-1,-1)而不是range(len(nums),0,-1)，左闭右开。
```
>>> a = [ i for i in range(10,0,-1)]
>>> a
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
>>> b = [i for i in range(10,-1,-1)]
>>> b
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
```


### 字典的**操作

