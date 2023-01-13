import math
import random

import outputfilter

fn = lambda x: math.sin(x)
values = [random.random() for _ in range(100)]

filter_mod = outputfilter.OutputFilter(fn, 0.5, -0.5)

print(filter_mod.run(values))