/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode* sortedArrayToBST(int* nums, int numsSize) {
	if (numsSize == 0) {
		return 0;
	}

	struct TreeNode* node = malloc(sizeof(struct TreeNode));
	if (numsSize == 1) {
		node->val = *nums;
		node->left = 0;
		node->right = 0;
		return node;
	}

	int half_idx = numsSize / 2;

	// Set the middle number to the node's value
	node->val = nums[half_idx];
	// printArr(nums, half_idx);
	// printArr(nums + half_idx + 1, numsSize - (half_idx + 1));
	// Sort all numbers up to but not including the middle number
	node->left = sortedArrayToBST(nums, half_idx);
	// Sort all numbers after the middle number
	node->right = sortedArrayToBST(nums + half_idx + 1, numsSize - (half_idx + 1));

	return node;
}