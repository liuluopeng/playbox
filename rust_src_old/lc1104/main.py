class Solution:
    def pathInZigZagTree(self, label: int) -> List[int]:
        res = []
        normal_direction = True
        start = 1 
        
        