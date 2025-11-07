pub fn fees_setting(
    initial_liquidity: u64,
    current_liquidity: u64,
    base_fee_bps: u64,
    proto_fee_bps: u64
) -> (u64, u64) {
    if initial_liquidity <= current_liquidity {
        let protocol_fee_bps = std::cmp::max(base_fee_bps / 100, proto_fee_bps);
        let swap_fee_bps = base_fee_bps - protocol_fee_bps;
        return (swap_fee_bps, protocol_fee_bps);
    }

    let deviation_bps = if current_liquidity > initial_liquidity {
        ((current_liquidity - initial_liquidity) * 10_000) / initial_liquidity
    } else {
        ((initial_liquidity - current_liquidity) * 10_000) / initial_liquidity
    };
    let deviation_bps = deviation_bps.min(10_000);

    let total_fee_bps = base_fee_bps + ((100_000 - base_fee_bps) * deviation_bps) / 10_000;

    let protocol_fee_bps = std::cmp::max(total_fee_bps / 100, proto_fee_bps);

    let swap_fee_bps = total_fee_bps - protocol_fee_bps;

    (swap_fee_bps, protocol_fee_bps)
}
