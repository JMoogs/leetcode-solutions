int* traverse(struct TreeNode* node, int* store) {
	if (node->left) {
		store = traverse(node->left, store);
	}
	*store = node->val;
	++store;
	if (node->right) {
		store = traverse(node->right, store);
	}
	return store;
}

int* inorderTraversal(struct TreeNode* root, int* returnSize) {
	int* arr = malloc(sizeof(int) * 100);
	if (root) {
		int* arr_end = traverse(root, arr);
		*returnSize = (arr_end - arr);
		return arr;
	} else {
		*returnSize = 0;
		return 0;
	}

}
