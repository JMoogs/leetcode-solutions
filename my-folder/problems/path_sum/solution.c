/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
int isLeaf(struct TreeNode*);

int hasPathSum(struct TreeNode* root, int targetSum) {
	// If we don't have a tree theres no path with the sum
	if (!root) {
		return 0;
	}
	// Stop if we hit a leaf
	if (isLeaf(root)) {
		return root->val == targetSum;
	}

	return hasPathSum(root->left, targetSum - root->val) || hasPathSum(root->right, targetSum - root->val);
}

int isLeaf(struct TreeNode* node) {
	return node && !node->left && !node->right;
}
