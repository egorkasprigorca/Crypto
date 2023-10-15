//
// Created by egor on 14.10.2023.
//

#ifndef CRYPTO_SHAMIR_H
#define CRYPTO_SHAMIR_H

#include <iostream>
#include <fstream>
#include <cmath>
#include <cstdlib>
#include <ctime>
#include <bitset>
#include <sstream>
#include "util.h"

namespace Shamir {
    void secret_key(int64_t *number1, int64_t *number2, int64_t p)
    {
        while ((*number1) * (*number2) % (p - 1) != 1)
        {
            *number1 = generate(1000, 10000, false);
            *number2 = generate(1000, 10000, false);
        }
    }

    void encode(
            int64_t cA, int64_t cB, int64_t dA, int64_t p,
            const std::string& inputFilePath, const std::string& outputFilePath
    ) {
        int64_t x1, x2;
        std::fstream in, out;
        in.open(inputFilePath, std::fstream::in | std::ios::binary);
        out.open(outputFilePath, std::fstream::out | std::ios::binary);
        char byte;
        while(in.get(byte)) {
            x1 = powmod(static_cast<int64_t>(static_cast<unsigned char>(byte)), cA, p);
            x2 = powmod(x1, cB, p);
            int64_t encryptedByte = powmod(x2, dA, p);
            out << encryptedByte << " ";
        }
        std::flush(in);
        std::flush(out);
        in.close();
        out.close();
    }

    void decode(int64_t dB, int64_t p, const std::string& codedFilePath, const std::string& outputFilePath)
    {
        std::fstream in, out;
        in.open(codedFilePath, std::fstream::in | std::ios::binary);
        out.open(outputFilePath, std::fstream::out | std::ios::binary);

        int64_t encryptedValue;
        while (in >> encryptedValue) {
            int64_t decryptedValue = powmod(encryptedValue, dB, p);
            out << static_cast<char>(decryptedValue);
        }
        std::flush(in);
        std::flush(out);
        in.close();
        out.close();
    }
};


#endif //CRYPTO_SHAMIR_H
