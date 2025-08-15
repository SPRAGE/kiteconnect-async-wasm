# Migration Guide: v1.0.9 API Spec Alignment

This release aligns the client to the official Kite Connect REST API. Non-spec helper/debug methods were removed and missing official endpoints were added.

Key changes:

- Added
  - POST /margins/orders -> `calculate_order_margins` and `calculate_order_margins_typed`
  - POST /margins/basket -> `calculate_basket_margins` and `calculate_basket_margins_typed`
  - POST /charges/orders -> `calculate_order_charges` and `calculate_order_charges_typed`
- Removed (non-spec helpers)
  - `instruments_simple`, `instruments_debug_json`, `instruments_with_gzip`, `instruments_typed_debug`
- Unchanged
  - All official endpoints retain signatures; typed variants end with `_typed`.

Code changes:

- Use `instruments(None)` or `instruments_typed(None)` instead of removed debug helpers.
- New typed models under `models::orders`:
  - `OrderMarginRequest`, `OrderMarginResult`, `BasketMarginsData`
  - `OrderChargesRequest`, `OrderChargesResult`, `ChargesBreakdown`, `GstBreakup`

Examples:

- See `examples/margin_calculation_example.rs` for margins/charges usage.

Notes:

- `OrderChargesRequest.average_price` is required (executed average price).
- Basket margins support `consider_positions` and `mode=compact`.
- Rate limits aligned to docs: Standard endpoints now 3 rps (Holdings, Positions, Profile, Instruments, etc.).
