class Solution:
    def shuffle(self, nums, n):
        z = list(zip(nums[:n],  nums[n:]))
        l = [list(i) for i in z]
        res = []
        for i in range(len(l)):
            res += l[i]
        return res


res = Solution()
print(res.shuffle([2, 5, 1, 3, 4, 7], 3))
