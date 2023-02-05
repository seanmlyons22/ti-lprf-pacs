#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "0:0\\]
Raw interrupt flag for DIO0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` reader - 0:0\\]
Raw interrupt flag for DIO0"]
pub type Dio0R = crate::BitReader<Dio0>;
impl Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio0 {
        match self.bits {
            true => Dio0::Set,
            false => Dio0::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio0::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio0::Clr
    }
}
#[doc = "1:1\\]
Raw interrupt flag for DIO1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` reader - 1:1\\]
Raw interrupt flag for DIO1"]
pub type Dio1R = crate::BitReader<Dio1>;
impl Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio1 {
        match self.bits {
            true => Dio1::Set,
            false => Dio1::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio1::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio1::Clr
    }
}
#[doc = "2:2\\]
Raw interrupt flag for DIO2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` reader - 2:2\\]
Raw interrupt flag for DIO2"]
pub type Dio2R = crate::BitReader<Dio2>;
impl Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio2 {
        match self.bits {
            true => Dio2::Set,
            false => Dio2::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio2::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio2::Clr
    }
}
#[doc = "3:3\\]
Raw interrupt flag for DIO3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` reader - 3:3\\]
Raw interrupt flag for DIO3"]
pub type Dio3R = crate::BitReader<Dio3>;
impl Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio3 {
        match self.bits {
            true => Dio3::Set,
            false => Dio3::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio3::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio3::Clr
    }
}
#[doc = "4:4\\]
Raw interrupt flag for DIO4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` reader - 4:4\\]
Raw interrupt flag for DIO4"]
pub type Dio4R = crate::BitReader<Dio4>;
impl Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio4 {
        match self.bits {
            true => Dio4::Set,
            false => Dio4::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio4::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio4::Clr
    }
}
#[doc = "5:5\\]
Raw interrupt flag for DIO5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` reader - 5:5\\]
Raw interrupt flag for DIO5"]
pub type Dio5R = crate::BitReader<Dio5>;
impl Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio5 {
        match self.bits {
            true => Dio5::Set,
            false => Dio5::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio5::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio5::Clr
    }
}
#[doc = "6:6\\]
Raw interrupt flag for DIO6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` reader - 6:6\\]
Raw interrupt flag for DIO6"]
pub type Dio6R = crate::BitReader<Dio6>;
impl Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio6 {
        match self.bits {
            true => Dio6::Set,
            false => Dio6::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio6::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio6::Clr
    }
}
#[doc = "7:7\\]
Raw interrupt flag for DIO7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` reader - 7:7\\]
Raw interrupt flag for DIO7"]
pub type Dio7R = crate::BitReader<Dio7>;
impl Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio7 {
        match self.bits {
            true => Dio7::Set,
            false => Dio7::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio7::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio7::Clr
    }
}
#[doc = "8:8\\]
Raw interrupt flag for DIO8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` reader - 8:8\\]
Raw interrupt flag for DIO8"]
pub type Dio8R = crate::BitReader<Dio8>;
impl Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio8 {
        match self.bits {
            true => Dio8::Set,
            false => Dio8::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio8::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio8::Clr
    }
}
#[doc = "9:9\\]
Raw interrupt flag for DIO9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` reader - 9:9\\]
Raw interrupt flag for DIO9"]
pub type Dio9R = crate::BitReader<Dio9>;
impl Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio9 {
        match self.bits {
            true => Dio9::Set,
            false => Dio9::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio9::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio9::Clr
    }
}
#[doc = "10:10\\]
Raw interrupt flag for DIO10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` reader - 10:10\\]
Raw interrupt flag for DIO10"]
pub type Dio10R = crate::BitReader<Dio10>;
impl Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio10 {
        match self.bits {
            true => Dio10::Set,
            false => Dio10::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio10::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio10::Clr
    }
}
#[doc = "11:11\\]
Raw interrupt flag for DIO11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` reader - 11:11\\]
Raw interrupt flag for DIO11"]
pub type Dio11R = crate::BitReader<Dio11>;
impl Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio11 {
        match self.bits {
            true => Dio11::Set,
            false => Dio11::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio11::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio11::Clr
    }
}
#[doc = "12:12\\]
Raw interrupt flag for DIO12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` reader - 12:12\\]
Raw interrupt flag for DIO12"]
pub type Dio12R = crate::BitReader<Dio12>;
impl Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio12 {
        match self.bits {
            true => Dio12::Set,
            false => Dio12::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio12::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio12::Clr
    }
}
#[doc = "13:13\\]
Raw interrupt flag for DIO13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` reader - 13:13\\]
Raw interrupt flag for DIO13"]
pub type Dio13R = crate::BitReader<Dio13>;
impl Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio13 {
        match self.bits {
            true => Dio13::Set,
            false => Dio13::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio13::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio13::Clr
    }
}
#[doc = "14:14\\]
Raw interrupt flag for DIO14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` reader - 14:14\\]
Raw interrupt flag for DIO14"]
pub type Dio14R = crate::BitReader<Dio14>;
impl Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio14 {
        match self.bits {
            true => Dio14::Set,
            false => Dio14::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio14::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio14::Clr
    }
}
#[doc = "15:15\\]
Raw interrupt flag for DIO15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` reader - 15:15\\]
Raw interrupt flag for DIO15"]
pub type Dio15R = crate::BitReader<Dio15>;
impl Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio15 {
        match self.bits {
            true => Dio15::Set,
            false => Dio15::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio15::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio15::Clr
    }
}
#[doc = "16:16\\]
Raw interrupt flag for DIO16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` reader - 16:16\\]
Raw interrupt flag for DIO16"]
pub type Dio16R = crate::BitReader<Dio16>;
impl Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio16 {
        match self.bits {
            true => Dio16::Set,
            false => Dio16::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio16::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio16::Clr
    }
}
#[doc = "17:17\\]
Raw interrupt flag for DIO17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` reader - 17:17\\]
Raw interrupt flag for DIO17"]
pub type Dio17R = crate::BitReader<Dio17>;
impl Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio17 {
        match self.bits {
            true => Dio17::Set,
            false => Dio17::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio17::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio17::Clr
    }
}
#[doc = "18:18\\]
Raw interrupt flag for DIO18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` reader - 18:18\\]
Raw interrupt flag for DIO18"]
pub type Dio18R = crate::BitReader<Dio18>;
impl Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio18 {
        match self.bits {
            true => Dio18::Set,
            false => Dio18::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio18::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio18::Clr
    }
}
#[doc = "19:19\\]
Raw interrupt flag for DIO19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` reader - 19:19\\]
Raw interrupt flag for DIO19"]
pub type Dio19R = crate::BitReader<Dio19>;
impl Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio19 {
        match self.bits {
            true => Dio19::Set,
            false => Dio19::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio19::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio19::Clr
    }
}
#[doc = "20:20\\]
Raw interrupt flag for DIO20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` reader - 20:20\\]
Raw interrupt flag for DIO20"]
pub type Dio20R = crate::BitReader<Dio20>;
impl Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio20 {
        match self.bits {
            true => Dio20::Set,
            false => Dio20::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio20::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio20::Clr
    }
}
#[doc = "21:21\\]
Raw interrupt flag for DIO21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` reader - 21:21\\]
Raw interrupt flag for DIO21"]
pub type Dio21R = crate::BitReader<Dio21>;
impl Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio21 {
        match self.bits {
            true => Dio21::Set,
            false => Dio21::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio21::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio21::Clr
    }
}
#[doc = "22:22\\]
Raw interrupt flag for DIO22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` reader - 22:22\\]
Raw interrupt flag for DIO22"]
pub type Dio22R = crate::BitReader<Dio22>;
impl Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio22 {
        match self.bits {
            true => Dio22::Set,
            false => Dio22::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio22::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio22::Clr
    }
}
#[doc = "23:23\\]
Raw interrupt flag for DIO23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` reader - 23:23\\]
Raw interrupt flag for DIO23"]
pub type Dio23R = crate::BitReader<Dio23>;
impl Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio23 {
        match self.bits {
            true => Dio23::Set,
            false => Dio23::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio23::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio23::Clr
    }
}
#[doc = "24:24\\]
Raw interrupt flag for DIO24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` reader - 24:24\\]
Raw interrupt flag for DIO24"]
pub type Dio24R = crate::BitReader<Dio24>;
impl Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio24 {
        match self.bits {
            true => Dio24::Set,
            false => Dio24::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio24::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio24::Clr
    }
}
#[doc = "25:25\\]
Raw interrupt flag for DIO25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` reader - 25:25\\]
Raw interrupt flag for DIO25"]
pub type Dio25R = crate::BitReader<Dio25>;
impl Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio25 {
        match self.bits {
            true => Dio25::Set,
            false => Dio25::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio25::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio25::Clr
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt flag for DIO0"]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt flag for DIO1"]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt flag for DIO2"]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw interrupt flag for DIO3"]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw interrupt flag for DIO4"]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Raw interrupt flag for DIO5"]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Raw interrupt flag for DIO6"]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Raw interrupt flag for DIO7"]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Raw interrupt flag for DIO8"]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Raw interrupt flag for DIO9"]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Raw interrupt flag for DIO10"]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Raw interrupt flag for DIO11"]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Raw interrupt flag for DIO12"]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Raw interrupt flag for DIO13"]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Raw interrupt flag for DIO14"]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Raw interrupt flag for DIO15"]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw interrupt flag for DIO16"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Raw interrupt flag for DIO17"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Raw interrupt flag for DIO18"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Raw interrupt flag for DIO19"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Raw interrupt flag for DIO20"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Raw interrupt flag for DIO21"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Raw interrupt flag for DIO22"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Raw interrupt flag for DIO23"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Raw interrupt flag for DIO24"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Raw interrupt flag for DIO25"]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {}
#[doc = "Raw interrupt flag for DIO pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
