use crate::{
    types::custom::PairMeta,
    utils::parser::account::AccountType
};


pub fn build_post(pair_meta: PairMeta) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(15);

    parts.push(format!(
        "📢 *BREAKING: NEW LIQUIDITY POOL DETECTED\\!* 🚀\n\n\
        🔥 *Market ID:* `{}`",
        pair_meta.market_id
    ));

    for (i, dev) in pair_meta.signers.iter().enumerate() {
        parts.push(format!("🛠️ *DEV {}:* `{}`", i + 1, dev));
    }

    for (i, shared_meta) in [pair_meta.base, pair_meta.quote].iter().enumerate() {
        let token_type: &str = if i == 0 { "BASE" } else { "QUOTE" };
        parts.push(format!("🔹 *{token_type} MINT:* `{}`", shared_meta.mint));
        if let Some(provided_liq_ratio) = shared_meta.provided_liq_ratio {
            let liq_level: &str = match provided_liq_ratio {
                liq if liq <= 10f64 => "LOW 🔴",
                liq if liq <= 30f64 => "MEDIUM 🟡",
                _ => "HIGH 🟢" 
            };
            parts.push(format!(
                "💰 *{token_type} provided LIQUIDITY:* *{}%* — *{}*", 
                format!("{:.2}", provided_liq_ratio).replace(".", "\\."),
                liq_level
            ));
        }
        if let Some(AccountType::Mint { mint_authority, freeze_authority, .. }) = &shared_meta.mint_account {
            parse_authority(&mut parts, format!("🔒 *{} MINT AUTHORITY*", token_type), mint_authority);
            parse_authority(&mut parts, format!("❄️ *{} FREEZE AUTHORITY*", token_type), freeze_authority);
        }
    }

    if let Some(raydium_related) = pair_meta.raydium_related {
        parts.push(format!("🌊 *RAYDIUM LP TOKEN MINT:* `{}`", raydium_related.lp_mint));
    }

    parts.join("\n\n")
}

fn parse_authority(parts: &mut Vec<String>, msg: String, authority: &Option<String>) -> () {
    match authority {
        Some(authority) => parts.push(format!("{msg}: `{}` — *RISKY 🔴*", authority)),
        None => parts.push(format!("{msg}: None — *SAFE 🟢*")),
    }
}