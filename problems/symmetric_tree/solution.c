int isMirror(struct TreeNode*, struct TreeNode*);

int isSymmetric(struct TreeNode* root) {
	return !root || isMirror(root->left, root->right);
}

int isMirror(struct TreeNode* tree1, struct TreeNode* tree2) {
	// A non-existent tree is always a mirror of itself
	// If only one tree exists they aren't mirrors
	// The value of each tree must be equal to be mirrors
	// If the tree has a left subtree, the other tree must have a right subtree, and vice versa
	// Finally check if the left subtree is a mirror of the right subtree and vice versa
	return (tree1 == tree2) || ((tree1 && tree2) && (tree1->val == tree2->val) && !(tree1->left && !(tree2->right))
		&& !(tree1->right && !(tree2->left)) && isMirror(tree1->left, tree2->right) && isMirror(tree1->right, tree2->left));

}
