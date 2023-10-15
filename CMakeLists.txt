cmake_minimum_required(VERSION 3.26)
project(Crypto)

set(CMAKE_CXX_STANDARD 17)

add_executable(Crypto lab2.cpp
        Shamir.h
        util.h
        RSA.h
        ELGamal.h
        Vernam.h
)
