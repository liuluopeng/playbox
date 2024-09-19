

# Definition for a binary tree node.
from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

        
class Solution:
    # 层序遍历 在遍历到下一层之前， 把数值保存。
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        queue = []
        
        next_level_counter = 0
        current_level_counter = 0

        if root is not None:
            queue.append(root)
            current_level_counter += 1
        
        # 同一层的节点列表：
        same_level = []
    
        while len(queue) != 0 :
            node = queue[0]
            print(node.val,  queue,"当前还有" ,current_level_counter,"下层已经累计" ,next_level_counter)
            queue.pop(0)
            current_level_counter -= 1

            if node.left is not None:
                queue.append(node.left)
                next_level_counter += 1
            if node.right is not None:
                queue.append(node.right)
                next_level_counter += 1
                
            if current_level_counter == 0:
                print("一层结束", node.val)
                res.append(node.val)
                current_level_counter = next_level_counter
                next_level_counter = 0
        print("res:",res)

        return []
    


if __name__ == "__main__":
    solution = Solution()


    left = TreeNode(val=22, left=None, right=None)
    right = TreeNode(val=33, left=None, right=None)
    root = TreeNode(val=11, left=left, right=right)
    solution.rightSideView(root=root)