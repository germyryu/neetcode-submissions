"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        # two pass algorithm
        # pass 1: create a hashmap where original node -> copy of original node with only val populated, no pointers populated
        # pass 2: for each original node, follow the next pointer, perform key lookup on that node, copy of original node.next is the value of the key lookup
        # same logic with the random
        store = {}
        curr = head
        while curr:
            store[curr] = Node(curr.val)
            curr = curr.next
        # now we have all the original node -> copy of node without pointers initialized in store map
        curr = head
        while curr:
            # copy of current original node
            curr_copy = store[curr]
            nxt = curr.next
            # lookup this node in store
            if nxt:
                nxt_copy = store[nxt]
                curr_copy.next = nxt_copy
            else:
                curr_copy.next = None

            rand = curr.random
            if rand:
                rand_copy = store[rand]
                curr_copy.random = rand_copy
            else:
                curr_copy.random = None

            curr = curr.next
        if head:
            return store[head]
        return None
