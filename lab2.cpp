#include <iostream>
#include <cstdlib>
#include <ctime>
#include "Shamir.h"
#include "RSA.h"
#include "ELGamal.h"
#include "Vernam.h"

const std::string dir = "C:/Users/Егор/Documents/Github/Crypto/data/";

void doShamir() {
    srand(time(NULL));
    int64_t cA, cB, dA, dB, p;
    p = generate(1e3, 1e4, true);
    // Alice
    Shamir::secret_key(&cA, &dA, p);
    // Bob
    Shamir::secret_key(&cB, &dB, p);

    Shamir::encode(cA, cB, dA, p,
                   dir + "logo.jpg",
                   dir + "coded_logo_shamir");
    Shamir::decode(dB, p,
                   dir + "coded_logo_shamir",
                   dir + "out_logo_shamir");
}

void doELGamal() {
    int64_t p, g, cB, Db, k, r;

    while (true) {
        p = generate(1e3, 1e4, true);
        g = generate(1e3, 1e4, true);
        cB = generate(1, p - 1, true);
        k = generate(1, p - 2, true);

        if ((g > 1) && (g < p - 1) && (cB > 1) && (cB < p - 1) && (1 <= k) && (k <= p - 2)) {
            break;
        }
    }

    r = ELGamal::encode(p, g, k, cB,
                        dir + "logo.jpg",
                        dir + "coded_logo_elgamal");
    ELGamal::decode(p, r, cB,
                    dir + "coded_logo_elgamal",
                    dir + "out_logo_elgamal.jpg");
}

void doVernam() {
    Vernam::encode(dir + "logo.jpg",
                   dir + "coded_logo_vernam",
                   dir + "vernam_key");
    Vernam::decode(dir + "coded_logo_vernam",
                   dir + "vernam_key",
                   dir + "out_logo_vernam.jpg");
}

void doRSA() {
    //    int64_t p = generate(0, 1000, true);
//    int64_t q = generate(0, 1000, true);
//    int64_t b = p * q;
//    int64_t fi = (p - 1) * (q - 1);
//    int64_t d, c;
//    while (true) {
//        d = generate(0, 1000, true);
//        if (d < fi and std::gcd(d, fi) == 1) {
//            break;
//        }
//    }
//    std::cout << "test" << std::endl;
//    while (true) {
//        c = generate(0, 1000, true);
//        if ((c * d) % fi == 1) {
//            break;
//        }
//    }

    long long p, q, Nb, fi, Db, Cb;

    while (1)
    {
        p = generate(0, 10000, true);
        q = generate(0, 10000, true);
        Nb = p * q;
        fi = (p - 1) * (q - 1);
        Db = generate(0, 10000, true);
        Cb = generate(0, 10000, true);
        if ((Db < fi) && (((Cb * Db) % fi) == 1))
            break;
    }

    RSA::encode(Db, Nb,
                dir + "logo.jpg",
                dir + "coded_logo_rsa");
    RSA::decode(Cb, Nb,
                dir + "coded_logo_rsa",
                dir + "out_logo_rsa.jpg");
//    int64_t x, y, min = 0;
//    gcd_ext(fi, d, &x, &y);
//    x = std::abs(x);
//    y = std::abs(y);
//    min = std::min(x, y);
//    c = fi - min;
//    int64_t g = c * d % fi;
//    std::cout << g << std::endl;
//    if (g == 1) {
//        std::cout << "ok" << std::endl;
//    }
}

int main() {
    doShamir();
    doELGamal();
    doVernam();
    doRSA();
}