class DynamicArray:
    
    def __init__(self, capacity: int):
        assert(capacity > 0)
        self.capacity = capacity
        self.arr = [None] * self.capacity
        self.tracker = 0

    def get(self, i: int) -> int:
        return self.arr[i]

    def set(self, i: int, n: int) -> None:
        self.arr[i] = n

    def pushback(self, n: int) -> None:
        # Check to see if the array is full
        if self.tracker >= self.capacity:
            # Store the original capacity of array
            x = self.capacity
            # Double the capacity of the array
            self.capacity *= 2
            for i in range(self.tracker, self.capacity):
                self.arr.append(None)
            self.arr[self.tracker] = n
        # If array is not full, place in array that is the first None
        else:
            self.arr[self.tracker] = n
        self.tracker += 1

    def popback(self) -> int:
        self.tracker -= 1
        res = self.arr[self.tracker]
        self.arr[self.tracker] = None
        return res

    def resize(self) -> None:
        self.capacity *= 2
        while len(self.arr) < self.capcity:
            self.arr.append(None)

    def getSize(self) -> int:
        return self.tracker
    
    def getCapacity(self) -> int:
        return self.capacity
