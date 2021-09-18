fn main() -> anyhow::Result<()> {
    let s = include_str!("input.txt");
    dbg!(&s[..40]);

    Ok(())
}
