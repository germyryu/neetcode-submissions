# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode()
        curr = dummy
        carry = 0
        while l1 or l2 or carry:
            if not l1:
                x = 0
            else:
                x = l1.val
            if not l2:
                y = 0
            else:
                y = l2.val
            total = x + y + carry
            carry = total // 10
            num = total % 10
            curr.next = ListNode(num)
            curr = curr.next
            if l1:
                l1 = l1.next
            if l2:
                l2 = l2.next
        return dummy.next