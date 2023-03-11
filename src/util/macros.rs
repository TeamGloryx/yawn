/// Matching at wicked high speeds
/// -- nothen, the developer of yawn
pub macro okmatch($thing: expr, $($rule: pat => $value:expr$(,)?)+) {
    match $thing {
        $($rule => $value,)+
    }
}

/// A macros allowing very fast and understandable (yop) boolean matching,
/// expanding to a `match` statement,
/// matching `true` to `$truth`,
/// everything else (`_`) to `$lie`.
pub macro yop($bool:expr => $truth:expr; $lie:expr) {
    match $bool {
        true => $truth,
        _ => $lie
    }
}

/// A macros allowing fluent and not painful at all, building of HashMaps.
/// This macro works like vec!, constructing a new HashMap,
/// then inserting key-value pairs into it.
pub macro map_of($($key:expr => $value:expr$(,)?)+) {
    {
        let mut __map = ::std::collections::HashMap::new();
        $(__map.insert($key, $value);)+
        __map
    }
}