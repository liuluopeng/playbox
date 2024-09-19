class Solution:
    def numJewelsInStones(self, J: str, S: str) -> int:
        count = 0
        js = [ch for ch in J]
        print(js)
        for ch in js :
            count += S.count(ch)
        return count


res = Solution()
print(res.numJewelsInStones("aA","aAAbbbb"))