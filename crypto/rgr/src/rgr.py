import random
from functools import cache
from cryptoMethods import *
from enum import Enum

from sympy import isprime

class Color(Enum):
    RED = 1
    BLUE = 2
    YELLOW = 3

# задание изначальной палитры цветов
pal = [Color.RED, Color.BLUE, Color.YELLOW]
# edges = [(1, 2), (2, 3), (2, 4), (2, 5), (4, 5), (5, 6), (3, 4)]
# vertices = 6
# colors = [Color.YELLOW, Color.BLUE, Color.YELLOW, Color.RED, Color.YELLOW, Color.BLUE]
    
# считывание файла с графом
file = open("C:/Users/egorkasprigorca/Documents/coding/projects/Crypto/crypto/rgr/src/data/graph","r")
lines = file.readlines()
num_line = 0
edges = list()
vertices = 0
colors = list()
for line in lines:
    parts = line.split(' ')
    if num_line == 0:
            vertices = int(parts[0])
    elif num_line == 1:
        for part in parts:
                match part.rstrip():
                    case "RED":
                        colors.append(Color.RED)
                    case "BLUE":
                        colors.append(Color.BLUE)
                    case "YELLOW":
                        colors.append(Color.YELLOW)     
    elif num_line >= 2:
        edges.append((int(parts[0]), int(parts[1])))
    num_line+=1

# случайная перестановка цветов    
random.shuffle(pal)
for i in range(len(colors)):
    match colors[i]:
        case Color.RED:
            colors[i] = pal[0]
        case Color.BLUE:
            colors[i] = pal[1]
        case Color.YELLOW:
            colors[i] = pal[2]

# параметры RSA
c_rsa = list()
d_rsa = list()
n_rsa = list()
p_rsa = list()
q_rsa = list()
for i in range(vertices):
    while True:
        q = generate_prime(0, 10 ** 9)
        p = 2 * q + 1
        if check_prime(p):
            break
    n = p*q
    fi = (p-1)*(q-1)
    while True:
        d = random.randint(0, fi-1)
        if gcd(fi, d) == 1:
            break
    c = inverseModule(d, fi)
    c_rsa.append(c)
    d_rsa.append(d)
    n_rsa.append(n)
    p_rsa.append(p)
    q_rsa.append(q)

# генерация числа r и замена битов
rs = list()
for i in range(vertices):
    r = random.randint(0, fi-1)
    match colors[i]:
        case Color.RED:
            right_bits = 0b00
        case Color.BLUE:
            right_bits = 0b01
        case Color.YELLOW:
            right_bits = 0b10
    left_bits = (r >> 2) << 2
    r = left_bits | right_bits
    rs.append(r)

# вычисление z
zs = list()
for i in range(vertices):
    z = binaryPow2(rs[i], d_rsa[i], n_rsa[i])
    zs.append(z)

# проверка 
for i in range(len(edges)):
    edge1 = edges[i][0]-1
    edge2 = edges[i][1]-1
    c1 = c_rsa[edge1]
    c2 = c_rsa[edge2]
    z1_ = binaryPow2(zs[edge1], c1, n_rsa[edge1])
    z2_ = binaryPow2(zs[edge2], c2, n_rsa[edge2])
    z1last2bits = z1_ & 0b11
    z2last2bits = z2_ & 0b11
    if z1last2bits == z2last2bits:
        break
    else:
        print("yes")
