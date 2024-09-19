### DFS

```go

var res []int

func preorderTraversal(root *TreeNode) []int {
	res = []int{}
	dfs(root)
	return res
}

func dfs(root *TreeNode) {
	if root != nil {
		res = append(res, root.Val) 
		dfs(root.Left)
		dfs(root.Right)
	}
}

作者：linbingyuan
链接：https://leetcode-cn.com/problems/binary-tree-preorder-traversal/solution/die-dai-di-gui-dfs-by-linbingyuan/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

```