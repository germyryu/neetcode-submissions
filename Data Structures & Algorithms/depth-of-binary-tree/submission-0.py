from collections import deque
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        visited = set()
        queue = deque([(root, 0)])
        max_depth = 0
        while queue:
            node, depth = queue.popleft()
            if not node:
                return max_depth
            max_depth = max(max_depth, depth)
            if node not in visited:
                visited.add(node)
                if node.left and node.left not in visited:
                    queue.append((node.left, depth+1))
                if node.right and node.right not in visited:
                    queue.append((node.right, depth+1))
        return max_depth+1