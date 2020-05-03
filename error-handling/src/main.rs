use rand::Rng;

fn main() {
    println!("result is {}", caller1());
}

fn caller5() -> Result<String, NewErr> {
    let n = gen2()?;
    Ok(n.to_string())
}

struct NewErr {
    ErrMsg: String,
}

impl std::convert::From<String> for NewErr {
    fn from(s: String) -> Self {
        NewErr { ErrMsg: s }
    }
}

fn caller4() -> Result<String, String> {
    let n = gen2()?;
    Ok(n.to_string())
}

fn caller3() -> Result<String, String> {
    let r = gen2();
    match r {
        Ok(i) => Ok(i.to_string()),
        Err(e) => Err(e),
    }
}

fn caller2() -> String {
    let r = gen2();
    match r {
        Ok(i) => i.to_string(),
        Err(e) => e,
    }
}

fn caller1() -> String {
    //gen2().unwrap()
    //gen2().unwrap_or(0)
    let n = gen2().expect("failed to generate valid number");
    return n.to_string();
}

fn gen2() -> Result<i32, String> {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1, 10);
    if num <= 5 {
        Ok(num)
    } else {
        Err(String::from(num.to_string() + " is more than 5"))
    }
}

fn gen1() -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 10);
    if num <= 5 {
        num
    } else {
        panic!("{} is more than 5", num);
    }
}
