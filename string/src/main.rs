fn split_string() {
    let str = "Let's have some fun";
    let iter = str.split(" ");
    for i in iter {
        println!("{}", i)
    }
}

fn str_and_string() {
    // &str
    let str_ref = "I'm str_ref";
    println!("{}", str_ref);

    // String
    let mut string = String::new();
    string.push_str("I'm string");
    println!("{}", string);

    let another_string = String::from("I'm another string");
    println!("{}", another_string);

    // from &str
    // by to_string()
    let string_from_str = "I'm string from &str".to_string();
    println!("{}", string_from_str);

    // by to_own() - best performance
    let string_from_str_to_owned = "I'm using to_owned()".to_owned();
    println!("{}", string_from_str_to_owned);

    // by into()
    let string_from_str_by_into: String = "I'm using into()".into();
    println!("{}", string_from_str_by_into);
}

fn substring() {
    let str = "ABCDEFG";

    // by slice
    let substr = &str[0..3];
    println!("{}", substr);

    // substring by using chars
    let mut chars = "ABCDEFG".chars();
    chars.by_ref().nth(3);
    let slice = chars.as_str();
    println!("{}", slice);
}

fn format() {
    let country_code = "+92";
    let phone_number = "123456789";

    let full_fone_number = format!("{}-{}", country_code, phone_number);
    println!("Phone number: {}", full_fone_number);
}

fn concat() {
    let concat_arr = ["super", "star"].concat();
    println!("{}", concat_arr);

    let concat_macro = concat!("super","star");
    println!("{}", concat_macro);

    let mut concat_string = String::from("super");
    concat_string = concat_string + "star";
    println!("{}", concat_string);
    
}

fn chars() {
    let chars = "🐊 🐼 😸 ".chars();
    for ch in chars {
        print!("{},", ch);
    }

}

fn string_function(s: &str) -> String {
    let mut res = s.to_string();
    res.push_str(" processed");
    res
}



fn main() {
    split_string();
    str_and_string();
    substring();
    format();
    concat();
    chars();
    
    println!();
    let s = string_function("call ");
    println!("{}", s);
    let s = string_function("call2 ");
    println!("{}", s);
}
