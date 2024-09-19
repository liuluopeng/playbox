from functools import reduce
import operator

def subtractProductAndSum(self, n):
    A = map(int, str(n))
    return reduce(operator.mul, A) - sum(A)