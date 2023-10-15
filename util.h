//
// Created by egor on 14.10.2023.
//

#ifndef CRYPTO_UTIL_H
#define CRYPTO_UTIL_H

#include <cstdint>
#include <cstdlib>

bool is_prime(int64_t number)
{
    if (number <= 1)
        return false;

    int64_t b = (int64_t)pow(number, 0.5);
    for (int64_t i = 2; i <= b; ++i)
    {
        if ((number % i) == 0)
            return false;
    }
    return true;
}

int64_t generate(int64_t minValue, int64_t maxValue, bool mode)
{
    if (mode == false)
        return (rand() % maxValue) + minValue;
    else
    {
        int64_t numberPrime;
        while (true)
        {
            numberPrime = (rand() % maxValue) + minValue;
            if (is_prime(numberPrime))
                return numberPrime;
        }
    }
}

int64_t powmod(int64_t a, int64_t x, int64_t p)
{
    int64_t r;
    int64_t y = 1;

    while (x > 0)
    {
        r = x % 2;
        x = x / 2;
        if (r == 1)
        {
            y = (y * a) % p;
        }
        a = a * a % p;
    }
    return y;
}

int64_t gcd(int64_t a, int64_t b) {
    if (b == 0) {
        return a;
    }

    int64_t x1, y1;
    int64_t ret = gcd(b, a % b);

    return ret;
}

int64_t gcd_ext(int64_t a, int64_t b, int64_t *x, int64_t *y) {
    if (a == 0) {
        *x = 0;
        *y = 1;
        return b;
    }

    int64_t x1, y1; // To store results of recursive call
    int64_t gcd = gcd_ext(b%a, a, &x1, &y1);

    // Update x and y using results of
    // recursive call
    *x = y1 - (b/a) * x1;
    *y = x1;

    return gcd;
}

#endif