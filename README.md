# Dynamic Slippage Config
This repository contains the `dynamic_slippage_config_json` that is used to estimate slippage at the `/swap` endpoint.

### Slippage Issues
1. Slippage threshold is not a one-size-fit-all solution -- Swapping different types of tokens require users to change slippage thresholds to cater to specific profiles.
2. Many different factors can contribute to slippage -- one of them is time, especially when users can take time to accept a quote/sign the transaction.

### How does Dynamic Slippage help
1. Based on the categories of the tokens being traded, Dynamic Slippage will use its category min max to calculate the optimal slippage.
2. Dynamic Slippage is also implemented at the `/swap` endpoint, this will allow slippage to be estimated after "Swap Click" instead of during quoting, which will bring the estimation closer to the actual swap execution.

**In the backend**
If `dynamicSlippage: true` is passed to the `/swap` endpoint, the backend will
1. Simulate a swap based on the exact quote.
2. Use the `quoteOutAmount` and `simulatedSwapOutAmount` to calculate a `simulatedSlippage`.
3. Use the `amplificationRatio` and multiply it to the `simulatedSlippage`.
4. With the latest `simulatedSlippage` value, we will apply some math to it.
  - Depending on the input/output tokens' categories, we will use the worse of category between the 2 tokens
  - This will dictate the `heuristicsMin` and `heuristicsMax`
  - Take the `results = min(simulatedSlippage, userMin, heuristicsMin)`
  - Take the `finalSlippage = max(results, userMax, heuristicsMax)`
5. Finally, the `/swap` endpoint will return the response already using the `finalSlippage` based on the the simulation and heuristics.

---

**Internal test stuff**

Validate dynamic slippage config

`cargo test`
