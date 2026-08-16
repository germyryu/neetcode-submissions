# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        l1 = list1
        l2 = list2
        result = ListNode()
        r = result
        while l1 and l2:
            if l1.val < l2.val:
                tmp = l1.next
                r.next = l1
                l1 = tmp
                r = r.next
            else:
                tmp = l2.next
                r.next = l2
                l2 = tmp
                r = r.next
        if l1:
            r.next = l1
        else:
            r.next = l2
        return result.next