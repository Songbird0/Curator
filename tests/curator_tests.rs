extern crate curator;

use curator::generator;


#[test]
#[should_panic(expected = "`gen_pwd` cannot be used if no flags were activated.")]
/// One password, zero flag.
fn pwd_gen_0_flag() {
    let mut cur: generator::Curator = match generator::Curator::new() {
        Ok(res) => res,
        Err(err) => panic!("Something went wrong: {}", err)
    };
    let pwd: String = cur.gen_pwd(10);
    println!("password: {}", &pwd);
    assert_eq!(pwd.len(), 10);
}

#[test]
/// One password, one flag.
fn pwd_gen_1_flag() {
    let mut cur: generator::Curator = match generator::Curator::new() {
        Ok(res) => res,
        Err(err) => panic!("Something went wrong: {}", err)
    };
    cur.enable_integer();
    let pwd: String = cur.gen_pwd(10);
    println!("password: {}", &pwd);
    assert_eq!(pwd.len(), 10);
}

#[test]
/// One password, two flags.
fn pwd_gen_2_flags() {
    let mut cur: generator::Curator = match generator::Curator::new() {
        Ok(res) => res,
        Err(err) => panic!("Something went wrong: {}", err)
    };
    cur.enable_integer().enable_lc();
    let pwd: String = cur.gen_pwd(10);
    println!("password: {}", &pwd);
    assert_eq!(pwd.len(), 10);
}

#[test]
/// One password, three flags.
fn pwd_gen_3_flags() {
    let mut cur: generator::Curator = match generator::Curator::new() {
        Ok(res) => res,
        Err(err) => panic!("Something went wrong: {}", err)
    };
    cur.enable_integer().enable_lc().enable_uc();
    let pwd: String = cur.gen_pwd(10);
    println!("password: {}", &pwd);
    assert_eq!(pwd.len(), 10);
}

#[test]
/// One password, four flags.
fn pwd_gen_4_flags() {
    let mut cur: generator::Curator = match generator::Curator::new() {
        Ok(res) => res,
        Err(err) => panic!("Something went wrong: {}", err)
    };
    cur.enable_integer().enable_lc().enable_uc().enable_spec();
    let pwd: String = cur.gen_pwd(10);
    println!("password: {}", &pwd);
    assert_eq!(pwd.len(), 10);
}
