struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) {
	struct ListNode* l1_current = list1;
	struct ListNode* l2_current = list2;
	struct ListNode* head;
	// Set head to the pointer to the smaller first value of list1 and list2
	if (l1_current && l2_current) {
		if (l1_current->val < l2_current->val) {
			head = l1_current;
			l1_current = l1_current->next;
		} else {
			head = l2_current;
			l2_current = l2_current->next;
		}

	// Trivial cases
	} else if (l1_current) {
		return list1;
	} else if (l2_current) {
		return list2;
	} else {
		return 0;
	}
	struct ListNode* current = head;
	while (1) {
		// We're finished
		if (!l1_current && !l2_current) {
			return head;
		}
		// Cases where we only have one list remaining
		// We can just assign the rest of the list to next here
		if (!l1_current) {
			current->next = l2_current;
			return head;
		}
		if (!l2_current) {
			current->next = l1_current;
			return head;
		}
		// Cases where we have two lists
		// Compare the values in each and add the smaller to the list being returned,
		// increasing pointers where necessary
		if (l1_current->val < l2_current->val) {
			current->next = l1_current;
			current = current->next;
			l1_current = l1_current->next;
		} else {
			current->next = l2_current;
			current = current->next;
			l2_current = l2_current->next;
		}
	}
}
