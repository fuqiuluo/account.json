pub fn group_code2uin(code: i64) -> i64 {
    let mut left = code / 1000000;
    if (0..=10).contains(&left) {
        left += 202
    } else if (11..=19).contains(&left) {
        left += 469
    } else if (20..=66).contains(&left) {
        left += 2080
    } else if (67..=156).contains(&left) {
        left += 1943
    } else if (157..=209).contains(&left) {
        left += 1990
    } else if (210..=309).contains(&left) {
        left += 3890
    } else if (310..=335).contains(&left) {
        left += 3490
    } else if (336..=386).contains(&left) {
        //335 336不确定
        left += 2265
    } else if (387..=499).contains(&left) {
        left += 3490
    }
    left * 1000000 + code % 1000000
}

pub fn group_uin2code(uin: i64) -> i64 {
    let mut left = uin / 1000000;
    if (202..=212).contains(&left) {
        left -= 202
    } else if (480..=488).contains(&left) {
        left -= 469
    } else if (2100..=2146).contains(&left) {
        left -= 2080
    } else if (2010..=2099).contains(&left) {
        left -= 1943
    } else if (2147..=2199).contains(&left) {
        left -= 1990
    } else if (2600..=2651).contains(&left) {
        left -= 2265
    } else if (3800..=3989).contains(&left) {
        left -= 3490
    } else if (4100..=4199).contains(&left) {
        left -= 3890
    }
    left * 1000000 + uin % 1000000
}