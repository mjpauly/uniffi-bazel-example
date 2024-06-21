from arithmetic import *

try:
    add(18446744073709551615, 1)
    assert(not("Should have thrown a IntegerOverflow exception!"))
except ArithmeticError.IntegerOverflow as e:
    print("Received expected overflow error. The numbers were {} and {}".format(e.a, e.b))
    # It's okay!
    pass

assert add(2, 4) == 6
assert sub(4, 2) == 2
assert div(8, 4) == 2
assert equal(2, 2)
assert not equal(4, 8)
