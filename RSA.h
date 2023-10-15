//
// Created by egor on 14.10.2023.
//

#ifndef CRYPTO_RSA_H
#define CRYPTO_RSA_H


#include <cstdint>
#include <string>
#include <fstream>
#include "util.h"

namespace RSA {
    void encode(
            int64_t d, int64_t n,
            const std::string& inputFilePath, const std::string& codedFilePath
            ) {
        std::fstream in, out;
        in.open(inputFilePath, std::fstream::in | std::ios::binary);
        out.open(codedFilePath, std::fstream::out | std::ios::binary);
        char byte;
        while (in.get(byte)) {
            out << powmod(byte, d, n) << " ";
        }
        std::flush(in);
        std::flush(out);
        in.close();
        out.close();
    }

    void decode(
            int64_t c, int64_t n,
            const std::string& codedFilePath, const std::string& outputFilePath
            ) {
        std::fstream in, out;
        in.open(codedFilePath, std::fstream::in | std::ios::binary);
        out.open(outputFilePath, std::fstream::out | std::ios::binary);
        int64_t enc;
        while (in >> enc) {
            int64_t decr = powmod(enc, c, n);
            out.put(static_cast<char>(decr));
        }
        std::flush(in);
        std::flush(out);
        in.close();
        out.close();
    }
};


#endif //CRYPTO_RSA_H
