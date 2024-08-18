#include <stdio.h>
#include <stdlib.h>
struct DynArray {
	int* mem;
	int size;
	int capacity;
};

struct DynArray* createDynArray(int);
void pushDynArray(struct DynArray*, int);
void preorder_recurse(struct DynArray*, struct Node*);

int* preorder(struct Node* root, int* returnSize) {
	struct DynArray* return_array = createDynArray(8);
	preorder_recurse(return_array, root);
	int* mem = return_array->mem;
	*returnSize = return_array->size;
	// Free the dynamic array structure after extracting the required info
	free((void*) return_array);
	return mem;
}

void preorder_recurse(struct DynArray* ret_arr, struct Node* tree) {
	if (!tree) {
		return;
	}
	
	// Root
	pushDynArray(ret_arr, tree->val);
	// Others from left to right
	for (int i = 0; i < tree->numChildren; ++i) {
		preorder_recurse(ret_arr, *(tree->children + i));
	}

}

struct DynArray* createDynArray(int start_size) {
	struct DynArray* arr = malloc(sizeof(struct DynArray));
	int* arr_mem = malloc(start_size * sizeof(int));

	arr->mem = arr_mem;
	arr->capacity = start_size;
	arr->size = 0;

	return arr;
}

void pushDynArray(struct DynArray* arr, int val) {
	if (arr->size == arr->capacity) {
		int* mem = (int*) realloc((void*) arr->mem, 2 * arr->capacity * sizeof(int));
		arr->mem = mem;
        arr->capacity *= 2;
	}
	*(arr->mem + arr->size) = val;
	++(arr->size);
	return;

}
