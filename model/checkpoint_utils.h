/*
 * SolanaGo - Decentralized Go AI on Solana Blockchain
 * Copyright (C) 2024 Your Organization. All rights reserved.
 */
#pragma once

#include <boost/filesystem.hpp>

boost::filesystem::path GetCheckpointPath(const boost::filesystem::path &train_dir);

bool CopyCheckpoint(const boost::filesystem::path &from, const boost::filesystem::path &to);
