struct ListNode* reverseList(struct ListNode* head) {
	struct ListNode* prev = head;
	if (!head) {
		return 0;
	}
	struct ListNode* current = head->next;
	if (!current) {
		prev->next = 0;
		return prev;
	}
	struct ListNode* next = current->next; 
	if (!next) {
		current->next = prev;
		prev->next = 0;
		return current;
	}

	prev->next = 0;
	current->next = prev;

	while (1) {
		// Start state
		// (0; NULL) (1; 0) (2; 3)
		prev = current;
		// (1; 0) (1; 0) (2; 3)
		current = next;
		// (1; 0) (2; 3) (2; 3)
		next = next->next;
		// (1; 0) (2; 3) (3; 4)
		current->next = prev;
		// End state
		// (1; 0) (2; 1) (3; 4)
		
		if (!next) {
			return current;
		}
	}
}
