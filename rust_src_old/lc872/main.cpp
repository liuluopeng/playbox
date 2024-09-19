/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

#include <vector>
using std::vector;

struct TreeNode
{
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

class Solution
{
public:
    bool leafSimilar(TreeNode *root1, TreeNode *root2)
    {
        vector<int> l1;
        vector<int> l2;

        dfs(root1, l1);
        dfs(root2, l2);

        if (l1.size() != l2.size())
        {
            return false;
        }

        for (int i = 0; i < l1.size(); i++)
        {
            if (l1[i] != l2[i])
            {
                return false;
            }
        }

        return true;
    }
    void dfs(struct TreeNode *root, vector<int> &result)
    {
        if (root == nullptr)
        {
            return;
        }

        dfs(root->left, result);

        if (root->left == nullptr && root->right == nullptr)
        {
            result.push_back(root->val);
        }

        dfs(root->right, result);
    }
};

int main()
{

    return 0;
}