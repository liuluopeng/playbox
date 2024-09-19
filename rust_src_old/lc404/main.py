# Definition for a binary tree node.
from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
 
        counter = 0
        
        def dfs(root: Optional[TreeNode] ):
            nonlocal counter 
            if root is not None:
                if root.left is not None and root.left.left is None and root.left.right is None:
                    counter += 1
                dfs(root.left)
                dfs(root.right)
        dfs(root=root)
        
        return counter
    


root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
root.left.left = TreeNode(4)
root.left.right = TreeNode(5)


s = Solution()
s.sumOfLeftLeaves(root)