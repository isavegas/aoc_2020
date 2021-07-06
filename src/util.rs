fn pairs<'a, S, V>(v: &'a Vec<S>, f: &dyn Fn(S, S) -> V) -> Vec<(&'a S, &'a S, V)> {
    let r = Vec::new();
    for i in 0..(v.len() - 1) {
        let s1 = v[i];
        let s2 = v[v.len() - (i + 1)];
        r.push((s1, s2, f(s1, s2)));
    }
    r
}
