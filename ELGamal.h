//
// Created by egor on 15.10.2023.
//

#ifndef CRYPTO_ELGAMAL_H
#define CRYPTO_ELGAMAL_H


#include <cstdint>
#include <string>
#include <fstream>
#include "util.h"

namespace ELGamal {
    int64_t encode(
            int64_t p, int64_t g, int64_t k, int64_t cB,
            const std::string& inputFilePath, const std::string& codedFilePath
            ) {
        std::fstream in, out;
        in.open(inputFilePath, std::fstream::in | std::ios::binary);
        out.open(codedFilePath, std::fstream::out | std::ios::binary);
        char byte;
        int64_t dB, r;
        while(in.get(byte)) {
            dB = powmod(g, cB, p);
            r = powmod(g, k, p);
            int64_t enc = (powmod(dB, k, p) * static_cast<int64_t>(byte)) % p;
            out << enc << " ";
        }
        std::flush(in);
        std::flush(out);
        in.close();
        out.close();
        return r;
    }

    void decode(
            int64_t p, int64_t r, int64_t cB,
            const std::string& codedFilePath, const std::string& outputFilePath
            ) {
        std::fstream in, out;
        in.open(codedFilePath, std::fstream::in | std::ios::binary);
        out.open(outputFilePath, std::fstream::out | std::ios::binary);
        int64_t enc;
        while (in >> enc) {
            int64_t decr = (powmod(r, (p - 1 - cB), p) * enc) % p;
            out << static_cast<char>(decr);
        }
        std::flush(in);
        std::flush(out);
        in.close();
        out.close();
    }
};


#endif //CRYPTO_ELGAMAL_H
