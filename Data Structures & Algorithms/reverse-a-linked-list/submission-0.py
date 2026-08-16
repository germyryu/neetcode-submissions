# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # prev stores the current node's next value
        prev = None
        # curr stores the current node we are looking at
        curr = head
        while curr:
            # store the next node since that will be the new curr after this iteration
            next_node = curr.next
            # the current node's next is prev
            curr.next = prev
            # the new prev is the current node we are looking at
            prev = curr
            # the new current node is the next node we stored
            curr = next_node
        # return prev here because curr will be None and prev will have the last node which is the new head
        return prev