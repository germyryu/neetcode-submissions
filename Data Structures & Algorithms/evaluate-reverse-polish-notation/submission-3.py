import math
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        valid_operators = set(['+', '*', '-', '/'])
        stack = []
        for x in tokens:
            print(stack)
            if not x in valid_operators:
                # it is a number
                num = int(x)
                stack.append(num)
            else:
                # it is an operator and we pop two elements and run the operator
                a = stack.pop()
                b = stack.pop()
                match x:
                    case '+':
                        stack.append(a+b)
                    case '*':
                        stack.append(a*b)
                    case '-':
                        stack.append(b-a)
                    case '/':
                        stack.append(math.trunc(b/a))
        return stack.pop()