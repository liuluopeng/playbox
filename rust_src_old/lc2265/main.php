<?php


class TreeNode
{
    public $val = null;
    public $left = null;
    public $right = null;

    public function __construct($val, $left, $right)
    {
        $this->val = $val;
        $this->left = $left;
        $this->right = $right;
    }
}


class Solution
{

    public $counter = 0;
    /**
     * @param TreeNode $root
     * @return Integer
     */
    function averageOfSubtree($root)
    {
        $this->dfs($root);
        return $this->counter;
    }

    // 统计个数 统计平均值
    function dfs($root)
    {
        if ($root != null) {

            $leftRes = $this->dfs($root->left);
            $rightRes = $this->dfs($root->right);

            $counterOfSubtree = $leftRes["counter"] + $rightRes["counter"] + 1;
            $sumOfSubtree = $leftRes["sum"] + $rightRes["sum"] + $root->val;

            var_dump($sumOfSubtree  ."  " . $counterOfSubtree);

            if ( (int)($sumOfSubtree / $counterOfSubtree ) == $root->val) {

                $this->counter += 1;
            }

            $res = array(
                "counter" => $counterOfSubtree,
                "sum" => $sumOfSubtree,
            );

            return $res;
        } else {


            $res = array(
                "counter" => 0,
                "sum" => 0,
            );

            return $res;
        }
    }

}




// $left = new TreeNode(1, 2, 3);
// $root = new TreeNode(0, $left, 2);


$root = new TreeNode(4,null, null);
$root->left = new TreeNode(8,null, null);
$root->left->left = new TreeNode(0,null, null);
$root->left->right = new TreeNode(1,null, null);
$root->right = new TreeNode(5,null, null);
$root->right->right = new TreeNode(6,null,  null);


var_dump($root);

$s = new Solution();
$ans = $s->averageOfSubtree($root);

var_dump($ans);

?>