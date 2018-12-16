use std::fmt::{ Display,Formatter,Error};
use core::ops::{ Add, Sub};
#[derive(Clone,Copy)]
pub struct Vec2{
    pub x:f32,
    pub y:f32
}

impl Vec2{
    pub fn new(x:f32,y:f32) ->Self{
        Vec2{x,y}
    }
    pub fn len(&self) ->f32 {
        (self.x.powf(2f32) + self.y.powf(2f32)).sqrt()
    }
    pub fn unitized(&mut self){
        let len = self.len();
        let ratio =  1.0f32/len;
        self.x *= ratio;
        self.y *= ratio;
    }
    pub fn dot_product(&self,oth:&Self) -> f32{
        self.x * oth.x + self.y * oth.y
    }
    pub fn multiply(&mut self,n:f32)
    {
        self.x *= n;
        self.y *= n;
    }
    pub fn angle(&self,other:&Self) -> f32
    {
        let mut oth = other.clone();
        oth.unitized();

        let mut sel = self.clone();
        sel.unitized();

        sel.dot_product(&oth).acos()
    }
    pub fn Projection(&self,oth:&Self)->Vec2{
        let v_len = self.angle(oth).cos() * self.len();
        let ration = v_len / oth.len();
        let mut ret = oth.clone();
        ret.multiply(ration);
        ret
    }
    pub fn mul_k(&self,n:f32)->Self{
        Vec2::new(n * self.x,n * self.y)
    }
}

impl Display for Vec2{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"x = {},y = {}",self.x,self.y);
        Ok(())
    }
}

impl Add for Vec2{
    type Output = Self;

    fn add(self, rhs: Self) -> <Self as Add>::Output {
        Vec2::new(self.x + rhs.x ,self.y + rhs.y)
    }
}

impl Sub for Vec2{
    type Output = Self;

    fn sub(self, rhs: Self) -> <Self as Sub>::Output {
        Vec2::new(self.x - rhs.x ,self.y - rhs.y)
    }
}



use std::fmt::Debug;
use std::ops::Mul;
#[derive(PartialEq)]
pub struct Mat2{
    m11:f32, m12:f32,
    m21:f32, m22:f32
}

impl Mat2{
    pub fn new(m11:f32,m12:f32,m21:f32,m22:f32)-> Mat2{
        Mat2{m11,m12,m21,m22}
    }
    pub fn unit() -> Mat2 {
        Mat2::new(1f32,0f32,0f32,1f32)
    }
    pub fn transposition(&self) ->Mat2{
        Mat2::new(self.m11,self.m21,self.m12,self.m22)
    }
    pub fn mul_k(&self,n:f32)->Mat2 {
        Mat2::new(n * self.m11,n * self.m12,n * self.m21,n * self.m22)
    }
    pub fn from_scale(v:Vec2)-> Mat2 {
        Mat2{
            m11:v.x,m12:0f32,
            m21:0f32,m22:v.y
        }
    }
}

impl Mul for Mat2{
    type Output = Mat2;

    fn mul(self, rhs: Self) -> <Self as Mul>::Output {
        Mat2{
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21, m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21, m22: self.m21 * rhs.m12 + self.m22 * rhs.m22
        }
    }
}
impl Mul<Vec2> for Mat2{
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> <Self as Mul<Vec2>>::Output {
        Vec2{
            x: rhs.x * self.m11 + rhs.y * self.m12,
            y: rhs.x * self.m21 + rhs.y * self.m22
        }
    }
}

impl Debug for Mat2{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"┌\t\t\t┐\n\
                  |\t{}\t{}\t|\n\
                  |\t{}\t{}\t|\n\
                  └\t\t\t┘",self.m11,self.m12,self.m21,self.m22);
        Ok(())
    }
}
