void swapElements(int*, int, int);
int removeElement(int* nums, int numsSize, int val) {
	if (numsSize == 0) { return 0; };
	if (numsSize == 1) {
		return *nums != val;
	}

	int current_idx = 0;
	int final_safe_pos = numsSize - 1;

	// Get final_safe_pos to a point where it is safe
	while (*(nums + final_safe_pos) == val) {
		--final_safe_pos;
		if (current_idx == final_safe_pos) {
			if (*nums == val) {
				return 0;
			} else {
				return 1;
			}
		}
	}
	
	// Start moving the index upwards
	while ((current_idx + 1) <= final_safe_pos) {
		if (*(nums + current_idx) == val) {
			swapElements(nums, current_idx, final_safe_pos);
			while (*(nums + final_safe_pos) == val) {
				if (final_safe_pos == 0) {
					return 0;
				}
				--final_safe_pos;
			}
		}
		++current_idx;
	}


	return final_safe_pos + 1;
	
}
void swapElements(int* nums, int idx1, int idx2) {
	int temp = *(nums + idx1);
	*(nums + idx1) = *(nums + idx2);
	*(nums + idx2) = temp;
	return;
}
