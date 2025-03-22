use crate::{
    types::custom::PairMeta,
    utils::parser::account::AccountType,
    constants::{
        NATIVE_MINT, USDC_MINT,
        NATIVE_DECIMALS, USDC_DECIMALS
    }
};


pub fn build_post_as_string(pair_meta: PairMeta) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(18);  // no re-allocation, if signers.len() <= 4

    parts.push(format!(
        "📢 *NEW LIQUIDITY POOL DETECTED\\!*\n\n\
        🔥 *Market ID:* `{}`",
        pair_meta.market_id
    ));

    parts.push(craft_block_separator("AUTHORITY"));

    for (i, dev) in pair_meta.signers.iter().enumerate() {
        parts.push(format!("🛠️ *DEV {}:* `{}`", i + 1, dev));
    }

    for (i, shared_meta) in [pair_meta.base, pair_meta.quote].iter().enumerate() {
        let token_type: &str = if i == 0 { "BASE TOKEN" } else { "QUOTE TOKEN" };

        parts.push(craft_block_separator(token_type));

        parts.push(format!("🔹 *MINT:* `{}`", shared_meta.mint));

        // using this flag in order to avoid checking authority for already safe tokens, such as SOL, USDC, etc.
        let skip_authority: bool = matches!(shared_meta.mint.as_str(), NATIVE_MINT | USDC_MINT);

        if let Some(msg) = match &shared_meta.mint {
            mint if mint == NATIVE_MINT => Some(craft_pooled_amount_msg(shared_meta.provided_liq_amount, TokenType::Sol)),
            mint if mint == USDC_MINT => Some(craft_pooled_amount_msg(shared_meta.provided_liq_amount, TokenType::Usdc)),
            _ => shared_meta.provided_liq_ratio.map(craft_pooled_percentage_msg)
        } {
            parts.push(msg);
        }
        
        if !skip_authority {
            if let Some(AccountType::Mint { mint_authority, freeze_authority, .. }) = &shared_meta.mint_account {
                parse_authority(&mut parts, "🔒 *MINT AUTHORITY*", mint_authority);
                parse_authority(&mut parts, "❄️ *FREEZE AUTHORITY*", freeze_authority);
            }
        }
    }

    if let Some(raydium_related) = pair_meta.raydium_related {
        parts.push(craft_block_separator("RAYDIUM RELATED"));
        parts.push(format!("🌊 *LP TOKEN MINT:* `{}`", raydium_related.lp_mint));
    }

    parts.join("\n\n")
}

fn craft_block_separator(header: &str) -> String {
    let mut s: String = String::with_capacity(10 + header.len());
    s.push_str("*————");
    s.push_str(header);
    s.push_str("————*");
    s
}

fn parse_authority(parts: &mut Vec<String>, msg: &str, authority: &Option<String>) -> () {
    match authority {
        Some(authority) => parts.push(format!("{msg}: `{}` — *RISKY 🔴*", authority)),
        None => parts.push(format!("{msg}: None — *SAFE 🟢*")),
    }
}

fn craft_pooled_amount_msg(amount: u64, token_type: TokenType) -> String {
    let (token, decimals) = match token_type {
        TokenType::Sol => ("SOL", NATIVE_DECIMALS),
        TokenType::Usdc => ("USDC", USDC_DECIMALS),
    };
    let unsafe_ui_amount: f64 = calc_ui_amount(amount, decimals);
    let markdownv2_safe_ui_amount: String = format_ui_amount_for_markdownv2(unsafe_ui_amount, None);
    format!("💰 *PROVIDED LIQUIDITY:* {} *{}*", markdownv2_safe_ui_amount, token)
}

fn craft_pooled_percentage_msg(provided_liq_ratio: f64) -> String {
    let liq_level: &str = match provided_liq_ratio {
        liq if liq <= 10f64 => "LOW 🔴",
        liq if liq <= 30f64 => "MEDIUM 🟡",
        _ => "HIGH 🟢" 
    };
    format!(
        "💰 *PROVIDED LIQUIDITY:* *{}%* — *{}*",
        format_ui_amount_for_markdownv2(provided_liq_ratio, Some(2)),
        liq_level
    )
}

fn calc_ui_amount(amount: u64, decimals: u8) -> f64 {
    amount as f64 / 10f64.powi(decimals as i32)
}

fn format_ui_amount_for_markdownv2(v: f64, round_to: Option<usize>) -> String {
    let unsafe_ui_amount: String = match round_to {
        Some(precision) => format!("{:.1$}", v, precision),
        None => format!("{}", v)
    };
    unsafe_ui_amount.replace(".", "\\.")
}

enum TokenType {
    Sol,
    Usdc
}