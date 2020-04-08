use rand::Rng;
fn get_rand_u32() -> Result<u32, String> {
    let mut rng = rand::thread_rng();
    let rand: u32 = rng.gen();
    Ok(rand)
}
fn get_rand_u16() -> Result<u16, String> {
    let mut rng = rand::thread_rng();
    let rand: u16 = rng.gen();
    Ok(rand)
}
fn get_rand_u8() -> Result<u8, String> {
    let mut rng = rand::thread_rng();
    let rand: u8 = rng.gen();
    Ok(rand)
}
pub fn get_total() -> Result<u32, Vec<String>> {
    let mut errs: Vec<String> = vec![];
    let results
    let int1 = get_rand_int();
    if int1.is_err() {errs.push(int1.unwrap_err());}
    let int2 = get_rand_int();
    if int2.is_err() { errs.push(int2.unwrap_err()); }
    let int3 = get_rand_int();
    if int3.is_err() {errs.push(int3.unwrap_err()); }
    if errs.len() > 0 {
        Err(errs)
    } else {
        Ok(int1.unwrap() + int2.unwrap() + int3.unwrap())
    }
}

