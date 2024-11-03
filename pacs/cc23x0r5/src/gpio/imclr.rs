#[doc = "Register `IMCLR` reader"]
pub type R = crate::R<ImclrSpec>;
#[doc = "Register `IMCLR` writer"]
pub type W = crate::W<ImclrSpec>;
#[doc = "0:0\\]
Clears DIO0 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` reader - 0:0\\]
Clears DIO0 in IMASK"]
pub type Dio0R = crate::BitReader<Dio0>;
impl Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio0 {
        match self.bits {
            true => Dio0::Clr,
            false => Dio0::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio0::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio0::NoEffect
    }
}
#[doc = "Field `DIO0` writer - 0:0\\]
Clears DIO0 in IMASK"]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dio0>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::NoEffect)
    }
}
#[doc = "1:1\\]
Clears DIO1 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` reader - 1:1\\]
Clears DIO1 in IMASK"]
pub type Dio1R = crate::BitReader<Dio1>;
impl Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio1 {
        match self.bits {
            true => Dio1::Clr,
            false => Dio1::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio1::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio1::NoEffect
    }
}
#[doc = "Field `DIO1` writer - 1:1\\]
Clears DIO1 in IMASK"]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dio1>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::NoEffect)
    }
}
#[doc = "2:2\\]
Clears DIO2 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` reader - 2:2\\]
Clears DIO2 in IMASK"]
pub type Dio2R = crate::BitReader<Dio2>;
impl Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio2 {
        match self.bits {
            true => Dio2::Clr,
            false => Dio2::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio2::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio2::NoEffect
    }
}
#[doc = "Field `DIO2` writer - 2:2\\]
Clears DIO2 in IMASK"]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dio2>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::NoEffect)
    }
}
#[doc = "3:3\\]
Clears DIO3 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` reader - 3:3\\]
Clears DIO3 in IMASK"]
pub type Dio3R = crate::BitReader<Dio3>;
impl Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio3 {
        match self.bits {
            true => Dio3::Clr,
            false => Dio3::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio3::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio3::NoEffect
    }
}
#[doc = "Field `DIO3` writer - 3:3\\]
Clears DIO3 in IMASK"]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dio3>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::NoEffect)
    }
}
#[doc = "4:4\\]
Clears DIO4 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` reader - 4:4\\]
Clears DIO4 in IMASK"]
pub type Dio4R = crate::BitReader<Dio4>;
impl Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio4 {
        match self.bits {
            true => Dio4::Clr,
            false => Dio4::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio4::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio4::NoEffect
    }
}
#[doc = "Field `DIO4` writer - 4:4\\]
Clears DIO4 in IMASK"]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dio4>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::NoEffect)
    }
}
#[doc = "5:5\\]
Clears DIO5 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` reader - 5:5\\]
Clears DIO5 in IMASK"]
pub type Dio5R = crate::BitReader<Dio5>;
impl Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio5 {
        match self.bits {
            true => Dio5::Clr,
            false => Dio5::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio5::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio5::NoEffect
    }
}
#[doc = "Field `DIO5` writer - 5:5\\]
Clears DIO5 in IMASK"]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dio5>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::NoEffect)
    }
}
#[doc = "6:6\\]
Clears DIO6 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` reader - 6:6\\]
Clears DIO6 in IMASK"]
pub type Dio6R = crate::BitReader<Dio6>;
impl Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio6 {
        match self.bits {
            true => Dio6::Clr,
            false => Dio6::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio6::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio6::NoEffect
    }
}
#[doc = "Field `DIO6` writer - 6:6\\]
Clears DIO6 in IMASK"]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dio6>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::NoEffect)
    }
}
#[doc = "7:7\\]
Clears DIO7 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` reader - 7:7\\]
Clears DIO7 in IMASK"]
pub type Dio7R = crate::BitReader<Dio7>;
impl Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio7 {
        match self.bits {
            true => Dio7::Clr,
            false => Dio7::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio7::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio7::NoEffect
    }
}
#[doc = "Field `DIO7` writer - 7:7\\]
Clears DIO7 in IMASK"]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dio7>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::NoEffect)
    }
}
#[doc = "8:8\\]
Clears DIO8 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` reader - 8:8\\]
Clears DIO8 in IMASK"]
pub type Dio8R = crate::BitReader<Dio8>;
impl Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio8 {
        match self.bits {
            true => Dio8::Clr,
            false => Dio8::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio8::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio8::NoEffect
    }
}
#[doc = "Field `DIO8` writer - 8:8\\]
Clears DIO8 in IMASK"]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dio8>;
impl<'a, REG> Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::NoEffect)
    }
}
#[doc = "9:9\\]
Clears DIO9 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` reader - 9:9\\]
Clears DIO9 in IMASK"]
pub type Dio9R = crate::BitReader<Dio9>;
impl Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio9 {
        match self.bits {
            true => Dio9::Clr,
            false => Dio9::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio9::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio9::NoEffect
    }
}
#[doc = "Field `DIO9` writer - 9:9\\]
Clears DIO9 in IMASK"]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dio9>;
impl<'a, REG> Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::NoEffect)
    }
}
#[doc = "10:10\\]
Clears DIO10 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` reader - 10:10\\]
Clears DIO10 in IMASK"]
pub type Dio10R = crate::BitReader<Dio10>;
impl Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio10 {
        match self.bits {
            true => Dio10::Clr,
            false => Dio10::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio10::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio10::NoEffect
    }
}
#[doc = "Field `DIO10` writer - 10:10\\]
Clears DIO10 in IMASK"]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dio10>;
impl<'a, REG> Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::NoEffect)
    }
}
#[doc = "11:11\\]
Clears DIO11 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` reader - 11:11\\]
Clears DIO11 in IMASK"]
pub type Dio11R = crate::BitReader<Dio11>;
impl Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio11 {
        match self.bits {
            true => Dio11::Clr,
            false => Dio11::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio11::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio11::NoEffect
    }
}
#[doc = "Field `DIO11` writer - 11:11\\]
Clears DIO11 in IMASK"]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dio11>;
impl<'a, REG> Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::NoEffect)
    }
}
#[doc = "12:12\\]
Clears DIO12 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` reader - 12:12\\]
Clears DIO12 in IMASK"]
pub type Dio12R = crate::BitReader<Dio12>;
impl Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio12 {
        match self.bits {
            true => Dio12::Clr,
            false => Dio12::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio12::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio12::NoEffect
    }
}
#[doc = "Field `DIO12` writer - 12:12\\]
Clears DIO12 in IMASK"]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dio12>;
impl<'a, REG> Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::NoEffect)
    }
}
#[doc = "13:13\\]
Clears DIO13 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` reader - 13:13\\]
Clears DIO13 in IMASK"]
pub type Dio13R = crate::BitReader<Dio13>;
impl Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio13 {
        match self.bits {
            true => Dio13::Clr,
            false => Dio13::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio13::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio13::NoEffect
    }
}
#[doc = "Field `DIO13` writer - 13:13\\]
Clears DIO13 in IMASK"]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dio13>;
impl<'a, REG> Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::NoEffect)
    }
}
#[doc = "14:14\\]
Clears DIO14 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` reader - 14:14\\]
Clears DIO14 in IMASK"]
pub type Dio14R = crate::BitReader<Dio14>;
impl Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio14 {
        match self.bits {
            true => Dio14::Clr,
            false => Dio14::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio14::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio14::NoEffect
    }
}
#[doc = "Field `DIO14` writer - 14:14\\]
Clears DIO14 in IMASK"]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dio14>;
impl<'a, REG> Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::NoEffect)
    }
}
#[doc = "15:15\\]
Clears DIO15 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` reader - 15:15\\]
Clears DIO15 in IMASK"]
pub type Dio15R = crate::BitReader<Dio15>;
impl Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio15 {
        match self.bits {
            true => Dio15::Clr,
            false => Dio15::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio15::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio15::NoEffect
    }
}
#[doc = "Field `DIO15` writer - 15:15\\]
Clears DIO15 in IMASK"]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dio15>;
impl<'a, REG> Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::NoEffect)
    }
}
#[doc = "16:16\\]
Clears DIO16 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` reader - 16:16\\]
Clears DIO16 in IMASK"]
pub type Dio16R = crate::BitReader<Dio16>;
impl Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio16 {
        match self.bits {
            true => Dio16::Clr,
            false => Dio16::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio16::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio16::NoEffect
    }
}
#[doc = "Field `DIO16` writer - 16:16\\]
Clears DIO16 in IMASK"]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dio16>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::NoEffect)
    }
}
#[doc = "17:17\\]
Clears DIO17 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` reader - 17:17\\]
Clears DIO17 in IMASK"]
pub type Dio17R = crate::BitReader<Dio17>;
impl Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio17 {
        match self.bits {
            true => Dio17::Clr,
            false => Dio17::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio17::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio17::NoEffect
    }
}
#[doc = "Field `DIO17` writer - 17:17\\]
Clears DIO17 in IMASK"]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dio17>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::NoEffect)
    }
}
#[doc = "18:18\\]
Clears DIO18 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` reader - 18:18\\]
Clears DIO18 in IMASK"]
pub type Dio18R = crate::BitReader<Dio18>;
impl Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio18 {
        match self.bits {
            true => Dio18::Clr,
            false => Dio18::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio18::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio18::NoEffect
    }
}
#[doc = "Field `DIO18` writer - 18:18\\]
Clears DIO18 in IMASK"]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dio18>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::NoEffect)
    }
}
#[doc = "19:19\\]
Clears DIO19 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` reader - 19:19\\]
Clears DIO19 in IMASK"]
pub type Dio19R = crate::BitReader<Dio19>;
impl Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio19 {
        match self.bits {
            true => Dio19::Clr,
            false => Dio19::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio19::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio19::NoEffect
    }
}
#[doc = "Field `DIO19` writer - 19:19\\]
Clears DIO19 in IMASK"]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dio19>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::NoEffect)
    }
}
#[doc = "20:20\\]
Clears DIO20 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` reader - 20:20\\]
Clears DIO20 in IMASK"]
pub type Dio20R = crate::BitReader<Dio20>;
impl Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio20 {
        match self.bits {
            true => Dio20::Clr,
            false => Dio20::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio20::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio20::NoEffect
    }
}
#[doc = "Field `DIO20` writer - 20:20\\]
Clears DIO20 in IMASK"]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dio20>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::NoEffect)
    }
}
#[doc = "21:21\\]
Clears DIO21 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` reader - 21:21\\]
Clears DIO21 in IMASK"]
pub type Dio21R = crate::BitReader<Dio21>;
impl Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio21 {
        match self.bits {
            true => Dio21::Clr,
            false => Dio21::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio21::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio21::NoEffect
    }
}
#[doc = "Field `DIO21` writer - 21:21\\]
Clears DIO21 in IMASK"]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dio21>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::NoEffect)
    }
}
#[doc = "22:22\\]
Clears DIO22 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` reader - 22:22\\]
Clears DIO22 in IMASK"]
pub type Dio22R = crate::BitReader<Dio22>;
impl Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio22 {
        match self.bits {
            true => Dio22::Clr,
            false => Dio22::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio22::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio22::NoEffect
    }
}
#[doc = "Field `DIO22` writer - 22:22\\]
Clears DIO22 in IMASK"]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dio22>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::NoEffect)
    }
}
#[doc = "23:23\\]
Clears DIO23 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` reader - 23:23\\]
Clears DIO23 in IMASK"]
pub type Dio23R = crate::BitReader<Dio23>;
impl Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio23 {
        match self.bits {
            true => Dio23::Clr,
            false => Dio23::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio23::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio23::NoEffect
    }
}
#[doc = "Field `DIO23` writer - 23:23\\]
Clears DIO23 in IMASK"]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dio23>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::NoEffect)
    }
}
#[doc = "24:24\\]
Clears DIO24 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` reader - 24:24\\]
Clears DIO24 in IMASK"]
pub type Dio24R = crate::BitReader<Dio24>;
impl Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio24 {
        match self.bits {
            true => Dio24::Clr,
            false => Dio24::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio24::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio24::NoEffect
    }
}
#[doc = "Field `DIO24` writer - 24:24\\]
Clears DIO24 in IMASK"]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dio24>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::NoEffect)
    }
}
#[doc = "25:25\\]
Clears DIO25 in IMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` reader - 25:25\\]
Clears DIO25 in IMASK"]
pub type Dio25R = crate::BitReader<Dio25>;
impl Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio25 {
        match self.bits {
            true => Dio25::Clr,
            false => Dio25::NoEffect,
        }
    }
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio25::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dio25::NoEffect
    }
}
#[doc = "Field `DIO25` writer - 25:25\\]
Clears DIO25 in IMASK"]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dio25>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::NoEffect)
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clears DIO0 in IMASK"]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears DIO1 in IMASK"]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears DIO2 in IMASK"]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears DIO3 in IMASK"]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clears DIO4 in IMASK"]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Clears DIO5 in IMASK"]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clears DIO6 in IMASK"]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clears DIO7 in IMASK"]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clears DIO8 in IMASK"]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clears DIO9 in IMASK"]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Clears DIO10 in IMASK"]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Clears DIO11 in IMASK"]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Clears DIO12 in IMASK"]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Clears DIO13 in IMASK"]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Clears DIO14 in IMASK"]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Clears DIO15 in IMASK"]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Clears DIO16 in IMASK"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Clears DIO17 in IMASK"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Clears DIO18 in IMASK"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Clears DIO19 in IMASK"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Clears DIO20 in IMASK"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Clears DIO21 in IMASK"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Clears DIO22 in IMASK"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Clears DIO23 in IMASK"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears DIO24 in IMASK"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clears DIO25 in IMASK"]
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
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clears DIO0 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio0(&mut self) -> Dio0W<ImclrSpec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears DIO1 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio1(&mut self) -> Dio1W<ImclrSpec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears DIO2 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio2(&mut self) -> Dio2W<ImclrSpec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears DIO3 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio3(&mut self) -> Dio3W<ImclrSpec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clears DIO4 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio4(&mut self) -> Dio4W<ImclrSpec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Clears DIO5 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio5(&mut self) -> Dio5W<ImclrSpec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clears DIO6 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio6(&mut self) -> Dio6W<ImclrSpec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Clears DIO7 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio7(&mut self) -> Dio7W<ImclrSpec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Clears DIO8 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio8(&mut self) -> Dio8W<ImclrSpec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clears DIO9 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio9(&mut self) -> Dio9W<ImclrSpec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Clears DIO10 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio10(&mut self) -> Dio10W<ImclrSpec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Clears DIO11 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio11(&mut self) -> Dio11W<ImclrSpec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Clears DIO12 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio12(&mut self) -> Dio12W<ImclrSpec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Clears DIO13 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio13(&mut self) -> Dio13W<ImclrSpec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Clears DIO14 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio14(&mut self) -> Dio14W<ImclrSpec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Clears DIO15 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio15(&mut self) -> Dio15W<ImclrSpec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Clears DIO16 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio16(&mut self) -> Dio16W<ImclrSpec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Clears DIO17 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio17(&mut self) -> Dio17W<ImclrSpec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Clears DIO18 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio18(&mut self) -> Dio18W<ImclrSpec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Clears DIO19 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio19(&mut self) -> Dio19W<ImclrSpec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Clears DIO20 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio20(&mut self) -> Dio20W<ImclrSpec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Clears DIO21 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio21(&mut self) -> Dio21W<ImclrSpec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Clears DIO22 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio22(&mut self) -> Dio22W<ImclrSpec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Clears DIO23 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio23(&mut self) -> Dio23W<ImclrSpec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears DIO24 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio24(&mut self) -> Dio24W<ImclrSpec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clears DIO25 in IMASK"]
    #[inline(always)]
    #[must_use]
    pub fn dio25(&mut self) -> Dio25W<ImclrSpec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<ImclrSpec> {
        Reserved26W::new(self, 26)
    }
}
#[doc = "Clear interrupt mask in IMASK by writing a one\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImclrSpec;
impl crate::RegisterSpec for ImclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imclr::R`](R) reader structure"]
impl crate::Readable for ImclrSpec {}
#[doc = "`write(|w| ..)` method takes [`imclr::W`](W) writer structure"]
impl crate::Writable for ImclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMCLR to value 0"]
impl crate::Resettable for ImclrSpec {
    const RESET_VALUE: u32 = 0;
}
