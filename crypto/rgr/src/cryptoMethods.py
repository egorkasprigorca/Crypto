import math
import random
from functools import cache

from sympy import isprime

# №1 - быстрое возведение в степень по модулю (рекурсивно и нет)
@cache
def binaryPow(a, x, p):
    if x == 1:
        return a % p
    if x & 1:
        return (binaryPow(a, x - 1, p) * a) % p
    res = binaryPow(a, x >> 1, p)
    res = res * res % p
    return res


def binaryPow2(a, x, p):
    res = 1
    a0 = a
    n = x
    while n:
        if n & 1:
            res = (res * a0) % p
        a0 = (a0 * a0) % p
        n >>= 1
    return res

# Алгоритм Евклида, для нахождения наибольшего общего делителя
def gcd(a, b):
    while b != 0:
        r = a % b
        a = b
        b = r
    return a

# генерируем взаимно-простое число
def generate_coprime(p):
    result = random.randint(2, p)
    while gcd(p, result) != 1:
        result = random.randint(2, p)
    return result

# генерируем простое число в указанных границах
def generate_prime(left, right):
    while True:
        p = random.randint(left, right)
        if check_prime(p):
            return p

# проверка на простоту числа используя теореме Ферма
def check_prime(p):
    if p <= 1:
        return False
    elif p == 2:
        return True
    a = random.randint(2, p - 1)
    if binaryPow2(a, (p - 1), p) != 1 or gcd(p, a) > 1:
        return False
    return True

# №2 - обобщённый алгоритм Евклида
@cache
def ex_gcd(a, b):
    if a == 0:
        return (b, 0, 1)
    d, x1, y1 = ex_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return (d, x, y)


# №3 - функция, возвращающая открытый ключ по схеме Диффи-Хеллмана
def DeffiHellmanOpenKey(g, X, p):
    return binaryPow(g, X, p)


def DeffiHellmanSystemKeys(g, p, X_A, X_B):
    Y_A = DeffiHellmanOpenKey(g, X_A, p)
    Y_B = DeffiHellmanOpenKey(g, X_B, p)

    Z_AB = binaryPow(Y_B, X_A, p)
    Z_BA = binaryPow(Y_A, X_B, p)

    return (Z_AB, Z_BA)

def inverseModule(a, m):
    d, x, y = ex_gcd(a, m)
    if d == 1:
        return (x % m + m) % m
    return None


# №4 - функция, шага младенца, шага великана
@cache
def babyStepGiantStep(a, y, p):
    k = m = math.floor(math.sqrt(p)) + 1
    baby_list = []
    giant_list = []
    baby = y
    for j in range(m):
        baby_list.append((baby, j))
        baby = (baby * a) % p
        # print(baby, (y*binaryPow(a,j+1,p))%p)
    baby = (baby * inverseModule(y, p)) % p
    giant = baby
    for i in range(k):
        giant_list.append((giant, i + 1))
        # print(giant,binaryPow(a,m*(i+1),p))
        giant = (giant * baby) % p

    baby_list.sort()
    giant_list.sort()
    # print(baby_list)
    # print(giant_list)
    res = []
    i = 0
    j = 0
    while i < k and j < m:
        if giant_list[i][0] < baby_list[j][0]:
            i += 1
        elif giant_list[i][0] > baby_list[j][0]:
            j += 1
        else:
            val = giant_list[i][1] * m - baby_list[j][1]
            if val < p:
                res.append(val)
            if (i < k - 1):
                i += 1
            else:
                j += 1
    return res


@cache
def phiFunc(n):
    if isprime(n):
        return n - 1
    result = n
    i = 2
    while i * i <= n:
        if n % i == 0:
            while n % i == 0:
                n //= i
            result -= result // n
    if n > 1:
        result -= result // n
    return result


def primitiveRoot(p):
    q = (p - 1) // 2
    if isprime(q):
        for g in range(2, p):
            if (binaryPow(g, q, p) != 1):
                return g

    phi = p - 1
    n = phi
    list_num = []
    i = 2
    while i * i <= n:
        if n % i == 0:
            list_num.append(i)
            while n % i == 0:
                n //= i
        i += 1
    if n > 1:
        list_num.append(i)

    for res in range(2, p):
        for num in list_num:
            if binaryPow(res, phi // num, p) == 1:
                break
        else:
            return res


def Eratosphen(n):
    nums = [True] * (n + 1)
    nums[0], nums[1] = False, False
    for i in range(2, n + 1):
        if nums[i]:
            if i * i <= n:
                for j in range(i * i, n + 1, i):
                    nums[j] = False
    return nums


def procedureFirst(x, c, t):
    two16 = 1 << 16
    y = [x]
    t_list = [t]
    while True:
        if (t_list[-1] > 16):
            t_list.append(t_list[-1] // 2)
        else:
            break
    s = len(t_list) - 1
    primes = Eratosphen(1 << t_list[-1])
    p = [0] * (s + 1)
    p[s] = primes.index(True, 1 << (t_list[-1] - 1))
    m = s - 1
    r = [0] * (m + 1)
    while m >= 0:
        r[m] = math.ceil(t_list[m] / 16)
        while True:
            flag = False
            y = [y[0]] * (r[m] + 1)
            for i, val in enumerate(y):
                y[i + 1] = (19381 * val + c) % two16
                if (i == r[m] - 1):
                    break
            Ym = 0
            for i, val in enumerate(y[:-1]):
                Ym += val * (1 << 0)
            y[0] = y[-1]
            N = math.ceil((1 << (t_list[m] - 1)) / p[m + 1]) + math.floor(
                (1 << (t_list[m] - 1)) * Ym / (p[m + 1] * (1 << (16 * r[m]))))
            if N & 1:
                N += 1
            k = 0
            while True:
                p[m] = p[m + 1] * (N + k) + 1
                if p[m] > (1 << t_list[m]):
                    break
                if binaryPow2(2, p[m + 1] * (N + k), p[m]) == 1 and binaryPow2(2, (N + k), p[m]) != 1:
                    flag = True
                    break
                k += 2
            if flag:
                m -= 1
                break
    return (p[0], p[1])


def procedureSecond(p, q):
    b = (p - 1) // q
    while True:
        d = random.randint(1, p - 1)
        a = binaryPow2(d, b, p)
        if a != 1:
            return a
