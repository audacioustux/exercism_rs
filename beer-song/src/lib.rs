pub fn verse(n: u32) -> String {
    // let verse_gen =

    // match n {
    //     0 => format!(
    //         "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"
    //     ),
    //     1 => format!(
    //         "{0} bottle of beer on the wall, {0} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
    //         n
    //     ),
    //     2 => format!(
    //         "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle of beer on the wall.\n",
    //         n, n-1
    //     ),
    //     _ => format!(
    //         "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",
    //         n, n-1
    //     ),
    // }

    let bottles = |x: u32| match x {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", x),
    };

    let wall = |x: Option<u32>| format!("{} of beer on the wall", bottles(x.unwrap_or(99)));
    let beer = format!("{} of beer", bottles(n));
    let action = match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    };

    format!(
        "{}, {}.\n{}, {}.\n",
        wall(Some(n)).first_char_to_uppercase(),
        beer,
        action,
        wall(n.checked_sub(1))
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = verse(start);

    for i in (end..start).rev() {
        song = format!("{}\n{}", song, verse(i));
    }
    song
}

trait StrUtils {
    fn first_char_to_uppercase(self) -> String;
}

impl StrUtils for String {
    fn first_char_to_uppercase(self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => self,
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
