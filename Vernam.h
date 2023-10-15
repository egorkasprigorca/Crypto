//
// Created by egor on 15.10.2023.
//

#ifndef CRYPTO_VERNAM_H
#define CRYPTO_VERNAM_H


#include <string>
#include <fstream>
#include <iostream>
#include "util.h"

namespace Vernam {
    void encode(const std::string& inputFilePath, const std::string& codedFilePath, const std::string& keyFilePath) {
        std::fstream in, out, key;
        in.open(inputFilePath, std::fstream::in | std::ios::binary);
        out.open(codedFilePath, std::fstream::out | std::ios::binary);
        key.open(keyFilePath, std::fstream::out | std::ios::binary);
        char byte;
        while (in.get(byte)) {
            int64_t k = generate(0, 255, false);
            char enc = byte ^ k;
            out << static_cast<int64_t>(enc) << " ";
            key << k << " ";
        }
        std::flush(in);
        std::flush(out);
        std::flush(key);
        in.close();
        out.close();
        key.close();
    }

    void decode(const std::string& codedFilePath, const std::string& keyFilePath, const std::string& outputFilePath) {
        std::fstream in, out, key;
        in.open(codedFilePath, std::fstream::in | std::ios::binary);
        out.open(outputFilePath, std::fstream::out | std::ios::binary);
        key.open(keyFilePath, std::fstream::in | std::ios::binary);
        int64_t enc, keyv;
        while (in >> enc && key >> keyv) {
            char decr = enc ^ keyv;
            out << decr;
        }
        std::flush(in);
        std::flush(out);
        std::flush(key);
        in.close();
        out.close();
        key.close();
    }
};


#endif //CRYPTO_VERNAM_H
