class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x<0:
            return False

        st = str(x)
        return st[::-1] == st[::]


s = Solution()
print(s.isPalindrome(-343))