import numpy as np
import scipy as sc
from time import time
x = np.arange(8192)
# print(x)
t = time()
print(sc.fft.dct(x, type=2))
print(f'{time() - t}s')