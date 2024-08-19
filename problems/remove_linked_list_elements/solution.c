struct ListNode* removeElements(struct ListNode* head, int val) {
	if (!head) {
		return 0;
	}

	// Find first non-val element and set it as the head
	while (head->val == val) {
		head = head->next;
		if (!head) {
			return 0;
		}
	}

	struct ListNode* last_valid = head;

	while (last_valid) {
		if (!last_valid->next) {
			return head;
		}
		if (last_valid->next->val == val) {
			last_valid->next = last_valid->next->next;
		} else {
			last_valid = last_valid->next;
		}
	}


	return head;


}
