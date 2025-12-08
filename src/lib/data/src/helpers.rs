pub(crate) const fn strip_prefix_or_self<'a>(s: &'a str, prefix: &str) -> &'a str {
    if prefix.is_empty() || prefix.len() > s.len() {
        return s;
    }

    let pb = prefix.as_bytes();
    let sb = s.as_bytes();
    let mut i = 0;
    while i < pb.len() {
        if sb[i] != pb[i] {
            return s;
        }
        i += 1;
    }
    // SAFTY: this won't panic becase the prefix is equal to the
    // valid utf8 string so we can split at the exact lenght of the string
    s.split_at(i).1
}
