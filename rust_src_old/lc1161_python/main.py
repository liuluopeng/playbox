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
    def maxLevelSum(self, root: Optional[TreeNode]) -> int:
        
        queue = []
        level_list = [] 
        if root != None :
            queue.append(root)
        while len(queue) > 0:
            tmp_queue = []
            level = []
            for node in queue:
                level.append(node.val)
                if node.left != None:
                    tmp_queue.append(node.left)
                if node.right != None:
                    tmp_queue.append(node.right)
            level_list.append(level)
            queue = tmp_queue
            level = []
            tmp_queue = []

        max = -2147483648
        res = -99
        for i in range(len(level_list)-1, -1, -1):
            sumaaa = sum(level_list[i])
            if max <= sumaaa:
                max = sumaaa 
                res = i 
        
    
        return res + 1 
    


st = "[1,7,0,7,-8,null,null]"
root = stringToTreeNode(st)
st2 = treeNodeToString(root)

s = Solution()
res = s.maxLevelSum(root)

print(res)