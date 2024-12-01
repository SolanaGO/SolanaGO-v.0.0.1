/*
 * SolanaGo - Decentralized Go AI on Solana Blockchain
 * Copyright (C) 2024 Your Organization. All rights reserved.
 */
#pragma once

#include <memory>

#include "model/zero_model_base.h"
#include "model/model_config.pb.h"

namespace tensorflow { class Session; }

class ZeroModel final : public ZeroModelBase
{
 public:
    ZeroModel(int gpu);
    ~ZeroModel();

    int Init(const ModelConfig &model_config) override;

    // input  [batch, 19 * 19 * 17]
    // policy [batch, 19 * 19 + 1]
    int Forward(const std::vector<std::vector<bool>> &inputs,
                std::vector<std::vector<float>> &policy, std::vector<float> &value) override;

    int GetGlobalStep(int &global_step) override;

    static void SetMKLEnv(const ModelConfig &model_config);

 private:
    std::unique_ptr<tensorflow::Session> m_session;
    int m_gpu;
};
