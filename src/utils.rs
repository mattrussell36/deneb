use web_sys::console;

pub fn console_log(s: String) {
    console::log_1(&s.into());
}

pub fn format_dp(p: &str, dp: usize) -> String {
    let p_len = p.len();
    if p_len <= dp {
        let padded_zeroes = "0".repeat(dp - p_len);
        return format!("0.{}{}", padded_zeroes, p);
    }
    let whole_part = &p[..p_len - dp];
    let fraction_parth = &p[p_len - dp..];
    format!("{}.{}", whole_part, fraction_parth)
}