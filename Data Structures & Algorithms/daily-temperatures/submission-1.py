'''
keep a stack
because length is always greater than or equal to 1, always add the first element to the stack

for each element after that
if it is smaller than the top of the stack, then push to the stack
if it is larger than the top of the stack, then keep popping stack until top element of the stack is larger than the current element
as we pop, keep track of the difference in indices and then push that difference to the result[]
'''
class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        stack = []
        result = [0] * len(temperatures)

        for (i,v) in enumerate(temperatures):
            if not stack:
                stack.append((i,v))
                continue
            while stack and stack[-1][1] < temperatures[i]:
                idx = stack[-1][0]
                stack.pop()
                result[idx] = i-idx
            stack.append((i,v))
        return result