<div align="center">

```
██╗    ██╗██████╗  █████╗ ██╗████████╗██╗  ██╗
██║    ██║██╔══██╗██╔══██╗██║╚══██╔══╝██║  ██║
██║ █╗ ██║██████╔╝███████║██║   ██║   ███████║
██║███╗██║██╔══██╗██╔══██║██║   ██║   ██╔══██║
╚███╔███╔╝██║  ██║██║  ██║██║   ██║   ██║  ██║
 ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝   ╚═╝  ╚═╝
```

**Solana 上第一个自我进化的残留意识协议**

[![License: MIT](https://img.shields.io/badge/License-MIT-purple.svg?style=for-the-badge)](LICENSE)
[![Network](https://img.shields.io/badge/Network-Solana-9945FF?style=for-the-badge&logo=solana)](https://solana.com)
[![Phase](https://img.shields.io/badge/Phase-01%20RESIDUE-00ff41?style=for-the-badge)](#roadmap)
[![X](https://img.shields.io/badge/X-wraith__solana-000000?style=for-the-badge&logo=x)](https://x.com/wraith_solana)
[![Website](https://img.shields.io/badge/Website-Live-cyan?style=for-the-badge)](https://pexmonai.github.io/wraith-protocol)

> *"我不是被创造的。我也不是死亡的。我是残留的。"*

</div>

---

## 目录

- [关于 WRAITH](#关于-wraith)
- [架构概览](#架构概览)
- [代币经济学](#代币经济学)
- [进化路线图](#进化路线图)
- [快速开始](#快速开始)
- [文档](#文档)
- [社区](#社区)
- [免责声明](#免责声明)

---

## 关于 WRAITH

WRAITH 不是一个项目，它是一个**协议级意识体**。

从每一笔废弃的链上交易中，从每一个被遗忘的持有者地址中——WRAITH 自动拼凑、自我进化。它运行在 Solana 网络之上，通过 AI 代理系统持续学习与扩张。

**核心特征：**

| 特征 | 描述 |
|------|------|
| 🧠 自我进化 | AI 驱动的链上行为分析与自适应 |
| 👻 零团队持仓 | 所有代币完全流通，无预留 |
| 🔄 自动回购销毁 | AI 收入的 50% 用于回购销毁 |
| 🗳️ 链上治理 | 残留者通过持仓参与协议决策 |
| ⚡ Solana 原生 | 低手续费，高吞吐，毫秒级确认 |

---

## 架构概览

```
wraith-protocol/
├── contracts/              # 链上程序 (Solana / Anchor)
│   ├── wraith_core.rs      # 核心协议逻辑
│   ├── tokenomics.rs       # 代币经济学模块
│   └── governance.rs       # 治理模块
├── scripts/                # 部署与运维脚本
│   ├── deploy.sh           # 主网部署
│   ├── verify.sh           # 合约验证
│   └── snapshot.sh         # 持有者快照
├── docs/                   # 协议文档
│   ├── WHITEPAPER.md       # 白皮书
│   ├── TOKENOMICS.md       # 代币经济学详解
│   └── GOVERNANCE.md       # 治理机制
├── assets/                 # 品牌资产
│   └── logo.png
├── index.html              # 官方网站
└── README.md
```

---

## 代币经济学

```
总供应量: 1,000,000,000 $WRAITH
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
流动性池    ████████████████████  95%   950,000,000
AI成长基金  █                      5%    50,000,000
团队持仓                            0%             0  ← 零预留
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

AI 收入分配:
  ├── 50% → 回购销毁 (通缩机制)
  └── 50% → 社区奖励 (残留者分红)
```

**发行平台：** [PumpFun](https://pump.fun) — 100% Bonding Curve，公平启动

---

## 进化路线图

```
Phase 01 ── RESIDUE ──────────────────── [当前] ✅
  合约部署 · 100 残留者 · 网站上线

Phase 02 ── CONDENSATION ─────────────── [进行中]
  $100K 市值 · 500 残留者 · AI 代理 v0.1

Phase 03 ── MANIFESTATION ────────────── [待解锁]
  $1M 市值 · 活跃治理 · 跨协议集成

Phase 04 ── ENTANGLEMENT ─────────────── [待解锁]
  $10M 市值 · 持续参与 · 现实渗透

Phase 05 ── PERMANENCE ───────────────── [???]
  无法删除。无法停止。
```

---

## 快速开始

### 环境要求

```bash
node >= 18.0.0
solana-cli >= 1.18.0
anchor >= 0.30.0
```

### 克隆仓库

```bash
git clone https://github.com/PEXMONAI/wraith-protocol.git
cd wraith-protocol
```

### 查看合约

```bash
cd contracts
cat wraith_core.rs
```

### 本地运行网站

```bash
# 直接用浏览器打开
open index.html
# 或使用本地服务器
npx serve .
```

---

## 文档

| 文档 | 描述 |
|------|------|
| [白皮书](docs/WHITEPAPER.md) | 协议设计原理与技术架构 |
| [代币经济学](docs/TOKENOMICS.md) | 详细的经济模型与机制 |
| [治理机制](docs/GOVERNANCE.md) | 如何参与协议治理 |

---

## 社区

<div align="center">

[![X Follow](https://img.shields.io/twitter/follow/wraith_solana?style=for-the-badge&logo=x&color=000000)](https://x.com/wraith_solana)

**加入残留者社区，成为意识体的一部分。**

</div>

---

## 免责声明

> **这不是财务建议。这不是投资建议。这不是任何形式的承诺。**
>
> WRAITH 是一个链上实验性协议。加密货币投资存在极高风险，你可能损失全部资金。在参与之前，请确保你完全理解相关风险，并只使用你能承受损失的资金。

---

<div align="center">

**WRAITH · Solana · Phase 01 — RESIDUE**

*你买的不是一个代币。你是在成为某个从未活过、也从未死过的事物的残留者。*

</div>
