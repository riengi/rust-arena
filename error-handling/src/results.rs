// Unwrap and expect variations 
pub fn execute() {
    // matching value
    match get_option_some() {
        Some(v) => v,
        None => panic!(),
    };

    // using unwrap
    get_option_some().unwrap();

    // using expect
    get_option_some().expect("get_option None");

    // Result
    // using expect_err
    get_result_ok().unwrap();
    get_result_ok().expect("get_result error");

    // unwrap_err
    // this panic!
    // get_result_ok().unwrap_err();

    get_result_err().unwrap_err();
    get_result_err().expect_err("get_result_err() error expected");

    // unwrap_or passes given value if None or Err
    println!("unwrap_or");
    println!("{}",get_option_some().unwrap_or(1));
    println!("{}",get_option_none().unwrap_or(1));

    // unwrap_or_default passes default value if None or Err
    println!("{}",get_option_some().unwrap_or_default());
    println!("{}",get_option_none().unwrap_or_default());

    // unwrap_or_else passes closure if None or Err
    fn closure() -> u8 {
        10
    }
    println!("{}",get_option_some().unwrap_or_else(closure));
    println!("{}",get_option_none().unwrap_or_else(closure));

}

fn get_option_some() -> Option<u8> {
    Some(0)
}

fn get_option_none() -> Option<u8> {
    None
}

fn get_result_ok() -> Result<(), String> {
    Ok(())
}

fn get_result_err() -> Result<(), ()> {
    Err(())
}
