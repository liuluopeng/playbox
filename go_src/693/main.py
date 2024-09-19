class Solution:
    def hasAlternatingBits(self, n: int) -> bool:
        n = bin(n)
        if '00' in n or '11' in n:
            return False
        return True