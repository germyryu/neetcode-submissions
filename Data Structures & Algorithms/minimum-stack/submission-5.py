# [5, 0, 2]
# 0, 0, 0
class MinStack:

    def __init__(self):
        self.stack = []
        self.prefix_stack = []

    def push(self, val: int) -> None:
        if not self.stack:
            min_val = val
        else:
            min_val = min(val, self.prefix_stack[-1])
        self.stack.append(val)
        self.prefix_stack.append(min_val)

    def pop(self) -> None:
        self.stack.pop()
        self.prefix_stack.pop()

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return self.prefix_stack[-1]
        
