class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        outer_l = 0
        outer_r = len(matrix)-1
        while outer_l <= outer_r:
            outer_mid = outer_l + ((outer_r - outer_l)//2)
            inner_l = 0
            inner_r = len(matrix[0])-1
            if target < matrix[outer_mid][inner_l]:
                outer_r = outer_mid-1
            elif target > matrix[outer_mid][inner_r]:
                outer_l = outer_mid+1
            else:
                while inner_l <= inner_r:
                    inner_mid = inner_l + ((inner_r - inner_l)//2)
                    if target < matrix[outer_mid][inner_mid]:
                        inner_r = inner_mid-1
                    elif target > matrix[outer_mid][inner_mid]:
                        inner_l = inner_mid+1
                    else:
                        return True
                return False
        return False