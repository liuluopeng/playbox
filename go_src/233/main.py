class Solution:
    def countDigitOne(self, n: int) -> int:
        bigSt = ""
        for i in range(n+1):
            bigSt += str(i)
        
        before = len(bigSt)
        
        # bigSt = bigSt.replace("1","")
        
        # after = len(bigSt)
        
        # return before - after

        print(before)


s = Solution()
print(s.countDigitOne(824883294))