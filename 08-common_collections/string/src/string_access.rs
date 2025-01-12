pub fn string_len_demo() {
    let len = String::from("اﻟﺴﻼمﻋﻠﻴﻜﻢ").len();
    println!("{}", len);
}

pub fn string_traversal_demo() {
    let s = String::from("aﻟﺴﻼمﻋﻠﻴﻜﻢ");
    for c in s.chars() {
        println!("{}", c);
    }
}