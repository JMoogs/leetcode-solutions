/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
#include <stdbool.h>

bool hasCycle(struct ListNode *head) {
    if (!head) { return false; };
	struct ListNode* fast = head;
	struct ListNode* slow = head;

	while (true) {
		fast = fast->next;
		if (!fast) { return false; };
		if (fast == slow) { return true; };
		fast = fast->next;
		if (!fast) { return false; };
		if (fast == slow) { return true; };
		slow = slow->next;
	}



}
