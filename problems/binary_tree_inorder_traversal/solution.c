

#include <stdio.h>

// struct TreeNode {
//      int val;
//      struct TreeNode *left;
//      struct TreeNode *right;
// };

typedef struct TreeNode TreeNode;


void traverse(TreeNode* tree, int* nums, int* returnSize) {


    if (tree->left) {
        traverse(tree->left, nums, returnSize);
        
    };

    // Add one after adding the value in the current tree.
    nums[(*returnSize)++]=tree->val;

    if (tree->right) {
        traverse(tree->right, nums, returnSize);
    }

}

int* inorderTraversal(struct TreeNode* root, int* returnSize){
    int* nums = (int*) malloc(100*sizeof(int));
    *returnSize = 0;
    
    if (root) {
        traverse(root, nums, returnSize);
    }
    
    nums = realloc(nums, sizeof(int) * (*returnSize));
    return nums;
}