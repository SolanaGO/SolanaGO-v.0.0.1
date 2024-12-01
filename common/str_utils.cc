/*
 * SolanaGo - Decentralized Go AI on Solana Blockchain
 * Copyright (C) 2024 Your Organization. All rights reserved.
 */
#include "str_utils.h"

std::vector<std::string> SplitStr(const std::string &str, char delim)
{
    std::vector<std::string> ret = {""};
    for (char c: str) {
        if (c == delim) {
            ret.emplace_back();
        } else {
            ret.back() += c;
        }
    }
    return ret;
}