from shelves.arithmetic import *

try:
    add(18446744073709551615, 1)
    assert(not("Should have thrown a IntegerOverflow exception!"))
except ArithmeticError.IntegerOverflow as e:
    # print("Received expected overflow error. The numbers were {} and {}".format(e.a, e.b))
    # It's okay!
    pass

assert add(2, 4) == 6
assert sub(4, 2) == 2
assert div(8, 4) == 2
assert equal(2, 2)
assert not equal(4, 8)

# test the subcrate export

assert hi().a == 64

from shelves.subcrate import *

assert get_string() == "secret"

import shelves.trig as trig

a = trig.one()
b = trig.one()
a.imag = 0.7
print(trig.to_string(trig.add(a, b)))

def approx(a, b):
    return abs(a - b) < 1e-10

# all should produce the same output
assert trig.Angle.from_degrees(360).dbg().degrees() == 360
assert trig.Angle.tau().mul(2).degrees() == 720
half_turn = trig.Angle.tau().mul(0.5).exp()
assert approx(half_turn.real, -1)
assert approx(half_turn.imag, 0)
quarter_turn = trig.Angle.tau().mul(0.25).exp()
full_turn = trig.mul(trig.mul(half_turn, quarter_turn), quarter_turn)
assert approx(full_turn.real, 1)
assert approx(full_turn.imag, 0)

assert not hasattr(trig.Angle.from_degrees(360), "hidden")
assert not hasattr(trig.Angle.from_degrees(360), "rad")
