pub fn line_from<'a>(loc: usize, lines: &'a Vec<usize>) -> Option<usize> {
    let mut line = None;

    let mut count = 0;
    for (i, len) in lines.iter().enumerate() {
        if loc < (count + len) {
            line = Some(i + 1);
            break;
        }

        count += *len;
    }

    line

    // alt impl:
    // let (line, _) = lines.iter().enumerate()
    //     .fold((None, 0), (|(is_found, count), (i, &len)| {
    //         let res = if let Some(_) = is_found {
    //             is_found
    //         } else {
    //             if x < (count + len) {
    //                 Some(i + 1)
    //             } else {
    //                 None
    //             }
    //         };
    //         (res, count + len)
    //     }));
}
