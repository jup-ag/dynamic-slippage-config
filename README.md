# Dynamic Slippage Configs
This repository contains configuration files that are used to estimate slippage at the `/swap` endpoint.

1. `slippage_config.json` contains the min max heuristics for different token categories.
2. `token_categories.json` contains the categorisations of tokens.

### How it works

1. We run a backend slippage estimator that uses real time and historical swap data to calculate and estimate slippage values.
2. The estimator then reads the token categories to determine which category the input/output tokens belong to.
3. Based on the categories, the estimator will apply the worse of min max heuristics between the 2 tokens on to the estimated slippage value.
4. The estimator will then return the final slippage value as part of the swap transaction in the `/swap` response.

---

**Internal test stuff**

Validate dynamic slippage config

`cargo test`
