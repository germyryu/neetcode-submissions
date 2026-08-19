import heapq
class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        heap = [-n for n in stones]
        # max heap
        heapq.heapify(heap)
        while len(heap) > 1:
            stone1 = heapq.heappop(heap)
            stone2 = heapq.heappop(heap)
            if stone1 == stone2:
                # both stones are destroyed
                continue
            # get the diff of the stones and then push
            diff = abs(abs(stone1) - abs(stone2))
            heapq.heappush(heap, -diff)
        if len(heap) == 1:
            return -heap[0]
        if len(heap) == 0:
            return 0