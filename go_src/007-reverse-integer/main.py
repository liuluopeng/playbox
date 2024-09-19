

class Solution:
    def reverse(self, x: int) -> int:

        # if x > 2**31-1 or x < -2**31:
        #     return 0

        if x > 2147483647 or x < -2147483648:
            return 0

        st = str(x)
        answer = 0

        if x > 0:
            answer = int(st[::-1])

        elif x < 0:
            answer = 0 - int(st[:0:-1])

        if answer > 2147483647 or answer < -2147483648:
            return 0
        else:
            return answer


s = Solution()
print(s.reverse(4444444))
