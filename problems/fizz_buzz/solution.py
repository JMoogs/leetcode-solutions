class Solution:
    def fizzBuzz(self, n: int) -> List[str]:
        l = [0] * n
        for i in range(1, n + 1):
            if i % 15 == 0:
                l[i-1] = "FizzBuzz"
            elif i % 5 == 0:
                l[i-1] = "Buzz"
            elif i % 3 == 0:
                l[i-1] = "Fizz"
            else:
                l[i-1] = str(i)
        return l
