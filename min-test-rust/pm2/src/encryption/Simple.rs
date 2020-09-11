

pub fn encode(str:&str) ->String
{
    let bs = str.as_bytes();
    let mut dst :Vec<u8> = Vec::new();
    let mut dir:i32 = 1;
    bs.iter().for_each(|it|{
        let mut n = 0u8;
        if dir > 0 && *it  == u8::max_value()
        {
            n = ((u8::min_value() as i32) + dir - 1) as u8;
        }else if dir < 0 && *it == u8::min_value()
        {
            n = ((u8::max_value() as i32) + dir + 1) as u8;
        }else {
            n = (*it as i32 + dir) as u8;
        }
        dst.push(n);
        if dir == 1 { dir = -1; } else {dir = 1;}
    });
    unsafe {
        String::from_utf8_unchecked(dst)
    }
}

pub fn decode(str:&str) ->String
{
    let bs = str.as_bytes();
    let mut dst :Vec<u8> = Vec::new();
    let mut dir:i32 = -1;
    bs.iter().for_each(|it|{
        let mut n = 0u8;
        if dir > 0 && *it  == u8::max_value()
        {
            n = ((u8::min_value() as i32) + dir - 1) as u8;
        }else if dir < 0 && *it == u8::min_value()
        {
            n = ((u8::max_value() as i32) + dir + 1) as u8;
        }else {
            n = (*it as i32 + dir) as u8;
        }
        dst.push(n);
        if dir == 1 { dir = -1; } else {dir = 1;}
    });
    unsafe {
        String::from_utf8_unchecked(dst)
    }
}