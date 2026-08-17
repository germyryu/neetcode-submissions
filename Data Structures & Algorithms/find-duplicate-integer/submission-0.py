class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        slow = fast = 0
        while True:
            slow = nums[slow]
            fast = nums[nums[fast]]
            if slow == fast:
                break
        # now slow == fast
        x = 0
        while True:
            slow = nums[slow]
            x = nums[x]
            if x == slow:
                return x