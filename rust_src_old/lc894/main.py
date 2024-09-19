# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
# Definition for a binary tree node.
from typing import List, Optional

def treeNodeToString(root):
    if not root:
        return "[]"
    output = ""
    queue = [root]
    current = 0
    while current != len(queue):
        node = queue[current]
        current = current + 1

        if not node:
            output += "null, "
            continue

        output += str(node.val) + ", "
        queue.append(node.left)
        queue.append(node.right)
    return "[" + output[:-2] + "]"

def stringToTreeNode(input):
    input = input.strip()
    input = input[1:-1]
    if not input:
        return None

    inputValues = [s.strip() for s in input.split(',')]
    root = TreeNode(int(inputValues[0]))
    nodeQueue = [root]
    front = 0
    index = 1
    while index < len(inputValues):
        node = nodeQueue[front]
        front = front + 1

        item = inputValues[index]
        index = index + 1
        if item != "null":
            leftNumber = int(item)
            node.left = TreeNode(leftNumber)
            nodeQueue.append(node.left)

        if index >= len(inputValues):
            break

        item = inputValues[index]
        index = index + 1
        if item != "null":
            rightNumber = int(item)
            node.right = TreeNode(rightNumber)
            nodeQueue.append(node.right)
    return root


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right



class Solution:
    def allPossibleFBT(self, n: int) -> List[Optional[TreeNode]]:
        res = []
        if n % 2  == 0:
            return [] 
        if n == 1:
            return [TreeNode(0)]
        # 去掉一个根节点后, 把n-1个分配在左右两侧
        # 比如7  去掉根节点还有6个  可以分配成  1 5     2 4     3 3     4 2     5 1 总共有5种  
        for i in range(1,n,2):
            left_cadi = self.allPossibleFBT(i)
            right_cadi = self.allPossibleFBT(n - i - 1)
            
            for l in left_cadi:
                for r in right_cadi:
                    one_res = TreeNode(0)
                    one_res.left = l 
                    one_res.right = r
                    # one_res = TreeNode(0, l, r) 
                    res.append(one_res)
        return res             
        


# class Solution:
#     def allPossibleFBT(self, n: int) -> List[Optional[TreeNode]]:
#         full_binary_trees = []
#         if n % 2 == 0:
#             return full_binary_trees
#         if n == 1:
#             full_binary_trees.append(TreeNode(0))
#             return full_binary_trees
#         for i in range(1, n, 2):
#             left_subtrees = self.allPossibleFBT(i)
#             right_subtrees = self.allPossibleFBT(n - 1 - i)
#             for left_subtree in left_subtrees:
#                 for right_subtree in right_subtrees:
#                     root = TreeNode(0, left_subtree, right_subtree)
#                     full_binary_trees.append(root)
#         return full_binary_trees


s = Solution()
n = 7
print(s.allPossibleFBT(7))

