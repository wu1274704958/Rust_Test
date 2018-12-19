#![allow(dead_code)]
#![allow(non_snake_case)]

#[cfg(test)]
mod tests;
mod syslv;
mod transform;
mod t1;
mod t2;



fn main()
{
    t2::test();
}