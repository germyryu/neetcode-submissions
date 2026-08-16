# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reverseLinkedList(self, head: Optional[ListNode]) -> ListNode:
        prev, curr = None, head
        while curr:
            next_node = curr.next
            curr.next = prev
            prev = curr
            curr = next_node
        return prev
    
    def reorderList(self, head: Optional[ListNode]) -> None:
        # find midpoint
        # using slow and fast pointer, the slow will be at the midpoint
        # when fast is at the end
        slow, fast = head, head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        mid = slow
        l1 = head
        l2 = mid
        # need to reverse l2
        reversed_l2 = self.reverseLinkedList(l2)
        # iterate through l1 and reverse l2 one by one starting with l1
        # initate dummy node for interim front of node
        front = f = ListNode()
        idx = 0
        while f != mid:
            if idx % 2 == 0:
                f.next = l1
                l1 = l1.next
            else:
                f.next = reversed_l2
                reversed_l2 = reversed_l2.next
            f = f.next
            idx += 1
    
