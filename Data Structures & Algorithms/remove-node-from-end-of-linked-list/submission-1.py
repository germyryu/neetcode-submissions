# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode()
        dummy.next=head
        l,r = head, head
        for _ in range(n):
            r = r.next
        prev = dummy
        while r:
            r = r.next
            prev = l
            l = l.next
        # now l is at the node that needs to be removed
        prev.next = l.next
        return dummy.next