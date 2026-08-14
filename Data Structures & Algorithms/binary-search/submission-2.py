class Solution:
    def search(self, nums: List[int], target: int) -> int:
        l = 0
        r = len(nums)
        while l <= r:
            mid = (l+r)//2
            if not mid < len(nums):
                return -1
            if target == nums[mid]:
                return mid
            elif target < nums[mid]:
                r = mid-1
            else:
                l = mid+1
        return -1