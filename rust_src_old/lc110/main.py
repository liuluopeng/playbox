# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
# Definition for a binary tree node.
from typing import Optional

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
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def get_depth(root, depth:int) -> int :
            if root is None:
                return depth 
            else:
                return max(get_depth(root.left, depth + 1),get_depth(root.right, depth+ 1))
        
        balanced = True 
        def find(root):
            if root is not None:
                left_depth = get_depth(root.left, 0)   
                right_depth = get_depth(root.right, 0)
                if abs(left_depth - right_depth) > 1:
                    nonlocal balanced
                    balanced = False 
                    return 
                else:
                    find(root.left)
                    find(root.right)
        find(root) 
        return balanced

st = "[3,9,20,null,null,15,7]"
st = "[1,2,2,3,3,null,null,4,4]"
root = stringToTreeNode(st)
s  = Solution()
print(s.isBalanced(root))