pub fn build_proverb(items: &[&str]) -> String {
    let mut proverb = Vec::with_capacity(items.len());

    let mut items_iter = items.iter().peekable();

    while let Some(want_word) = items_iter.next() {
        if let Some(lost_word) = items_iter.peek() {
            proverb.push(format!(
                "For want of a {} the {} was lost.",
                want_word, lost_word
            ))
        } else {
            proverb.push(format!("And all for the want of a {}.", items[0]))
        }
    }
    proverb.join("\n")
}
