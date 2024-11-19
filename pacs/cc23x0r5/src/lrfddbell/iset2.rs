#[doc = "Register `ISET2` reader"]
pub type R = crate::R<Iset2Spec>;
#[doc = "Register `ISET2` writer"]
pub type W = crate::W<Iset2Spec>;
#[doc = "0:0\\]
PBE0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe0 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe0> for bool {
    #[inline(always)]
    fn from(variant: Pbe0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE0` reader - 0:0\\]
PBE0 event"]
pub type Pbe0R = crate::BitReader<Pbe0>;
impl Pbe0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe0 {
        match self.bits {
            true => Pbe0::Set,
            false => Pbe0::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe0::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe0::Noeff
    }
}
#[doc = "Field `PBE0` writer - 0:0\\]
PBE0 event"]
pub type Pbe0W<'a, REG> = crate::BitWriter<'a, REG, Pbe0>;
impl<'a, REG> Pbe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe0::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe0::Noeff)
    }
}
#[doc = "1:1\\]
PBE1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe1 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe1> for bool {
    #[inline(always)]
    fn from(variant: Pbe1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE1` reader - 1:1\\]
PBE1 event"]
pub type Pbe1R = crate::BitReader<Pbe1>;
impl Pbe1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe1 {
        match self.bits {
            true => Pbe1::Set,
            false => Pbe1::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe1::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe1::Noeff
    }
}
#[doc = "Field `PBE1` writer - 1:1\\]
PBE1 event"]
pub type Pbe1W<'a, REG> = crate::BitWriter<'a, REG, Pbe1>;
impl<'a, REG> Pbe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe1::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe1::Noeff)
    }
}
#[doc = "2:2\\]
PBE2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe2 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe2> for bool {
    #[inline(always)]
    fn from(variant: Pbe2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE2` reader - 2:2\\]
PBE2 event"]
pub type Pbe2R = crate::BitReader<Pbe2>;
impl Pbe2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe2 {
        match self.bits {
            true => Pbe2::Set,
            false => Pbe2::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe2::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe2::Noeff
    }
}
#[doc = "Field `PBE2` writer - 2:2\\]
PBE2 event"]
pub type Pbe2W<'a, REG> = crate::BitWriter<'a, REG, Pbe2>;
impl<'a, REG> Pbe2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe2::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe2::Noeff)
    }
}
#[doc = "3:3\\]
PBE3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe3 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe3> for bool {
    #[inline(always)]
    fn from(variant: Pbe3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE3` reader - 3:3\\]
PBE3 event"]
pub type Pbe3R = crate::BitReader<Pbe3>;
impl Pbe3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe3 {
        match self.bits {
            true => Pbe3::Set,
            false => Pbe3::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe3::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe3::Noeff
    }
}
#[doc = "Field `PBE3` writer - 3:3\\]
PBE3 event"]
pub type Pbe3W<'a, REG> = crate::BitWriter<'a, REG, Pbe3>;
impl<'a, REG> Pbe3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe3::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe3::Noeff)
    }
}
#[doc = "4:4\\]
PBE4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe4 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe4> for bool {
    #[inline(always)]
    fn from(variant: Pbe4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE4` reader - 4:4\\]
PBE4 event"]
pub type Pbe4R = crate::BitReader<Pbe4>;
impl Pbe4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe4 {
        match self.bits {
            true => Pbe4::Set,
            false => Pbe4::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe4::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe4::Noeff
    }
}
#[doc = "Field `PBE4` writer - 4:4\\]
PBE4 event"]
pub type Pbe4W<'a, REG> = crate::BitWriter<'a, REG, Pbe4>;
impl<'a, REG> Pbe4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe4::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe4::Noeff)
    }
}
#[doc = "5:5\\]
PBE5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe5 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe5> for bool {
    #[inline(always)]
    fn from(variant: Pbe5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE5` reader - 5:5\\]
PBE5 event"]
pub type Pbe5R = crate::BitReader<Pbe5>;
impl Pbe5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe5 {
        match self.bits {
            true => Pbe5::Set,
            false => Pbe5::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe5::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe5::Noeff
    }
}
#[doc = "Field `PBE5` writer - 5:5\\]
PBE5 event"]
pub type Pbe5W<'a, REG> = crate::BitWriter<'a, REG, Pbe5>;
impl<'a, REG> Pbe5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe5::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe5::Noeff)
    }
}
#[doc = "6:6\\]
PBE6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe6 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe6> for bool {
    #[inline(always)]
    fn from(variant: Pbe6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE6` reader - 6:6\\]
PBE6 event"]
pub type Pbe6R = crate::BitReader<Pbe6>;
impl Pbe6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe6 {
        match self.bits {
            true => Pbe6::Set,
            false => Pbe6::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe6::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe6::Noeff
    }
}
#[doc = "Field `PBE6` writer - 6:6\\]
PBE6 event"]
pub type Pbe6W<'a, REG> = crate::BitWriter<'a, REG, Pbe6>;
impl<'a, REG> Pbe6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe6::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe6::Noeff)
    }
}
#[doc = "7:7\\]
PBE7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe7 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe7> for bool {
    #[inline(always)]
    fn from(variant: Pbe7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE7` reader - 7:7\\]
PBE7 event"]
pub type Pbe7R = crate::BitReader<Pbe7>;
impl Pbe7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe7 {
        match self.bits {
            true => Pbe7::Set,
            false => Pbe7::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe7::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe7::Noeff
    }
}
#[doc = "Field `PBE7` writer - 7:7\\]
PBE7 event"]
pub type Pbe7W<'a, REG> = crate::BitWriter<'a, REG, Pbe7>;
impl<'a, REG> Pbe7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe7::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe7::Noeff)
    }
}
#[doc = "8:8\\]
PBE8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe8 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe8> for bool {
    #[inline(always)]
    fn from(variant: Pbe8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE8` reader - 8:8\\]
PBE8 event"]
pub type Pbe8R = crate::BitReader<Pbe8>;
impl Pbe8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe8 {
        match self.bits {
            true => Pbe8::Set,
            false => Pbe8::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe8::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe8::Noeff
    }
}
#[doc = "Field `PBE8` writer - 8:8\\]
PBE8 event"]
pub type Pbe8W<'a, REG> = crate::BitWriter<'a, REG, Pbe8>;
impl<'a, REG> Pbe8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe8::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe8::Noeff)
    }
}
#[doc = "9:9\\]
PBE9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe9 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe9> for bool {
    #[inline(always)]
    fn from(variant: Pbe9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE9` reader - 9:9\\]
PBE9 event"]
pub type Pbe9R = crate::BitReader<Pbe9>;
impl Pbe9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe9 {
        match self.bits {
            true => Pbe9::Set,
            false => Pbe9::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe9::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe9::Noeff
    }
}
#[doc = "Field `PBE9` writer - 9:9\\]
PBE9 event"]
pub type Pbe9W<'a, REG> = crate::BitWriter<'a, REG, Pbe9>;
impl<'a, REG> Pbe9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe9::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe9::Noeff)
    }
}
#[doc = "10:10\\]
PBE10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe10 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe10> for bool {
    #[inline(always)]
    fn from(variant: Pbe10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE10` reader - 10:10\\]
PBE10 event"]
pub type Pbe10R = crate::BitReader<Pbe10>;
impl Pbe10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe10 {
        match self.bits {
            true => Pbe10::Set,
            false => Pbe10::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe10::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe10::Noeff
    }
}
#[doc = "Field `PBE10` writer - 10:10\\]
PBE10 event"]
pub type Pbe10W<'a, REG> = crate::BitWriter<'a, REG, Pbe10>;
impl<'a, REG> Pbe10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe10::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe10::Noeff)
    }
}
#[doc = "11:11\\]
PBE11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe11 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe11> for bool {
    #[inline(always)]
    fn from(variant: Pbe11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE11` reader - 11:11\\]
PBE11 event"]
pub type Pbe11R = crate::BitReader<Pbe11>;
impl Pbe11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe11 {
        match self.bits {
            true => Pbe11::Set,
            false => Pbe11::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe11::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe11::Noeff
    }
}
#[doc = "Field `PBE11` writer - 11:11\\]
PBE11 event"]
pub type Pbe11W<'a, REG> = crate::BitWriter<'a, REG, Pbe11>;
impl<'a, REG> Pbe11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe11::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe11::Noeff)
    }
}
#[doc = "12:12\\]
PBE12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe12 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe12> for bool {
    #[inline(always)]
    fn from(variant: Pbe12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE12` reader - 12:12\\]
PBE12 event"]
pub type Pbe12R = crate::BitReader<Pbe12>;
impl Pbe12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe12 {
        match self.bits {
            true => Pbe12::Set,
            false => Pbe12::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe12::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe12::Noeff
    }
}
#[doc = "Field `PBE12` writer - 12:12\\]
PBE12 event"]
pub type Pbe12W<'a, REG> = crate::BitWriter<'a, REG, Pbe12>;
impl<'a, REG> Pbe12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe12::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe12::Noeff)
    }
}
#[doc = "13:13\\]
PBE13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe13 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe13> for bool {
    #[inline(always)]
    fn from(variant: Pbe13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE13` reader - 13:13\\]
PBE13 event"]
pub type Pbe13R = crate::BitReader<Pbe13>;
impl Pbe13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe13 {
        match self.bits {
            true => Pbe13::Set,
            false => Pbe13::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe13::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe13::Noeff
    }
}
#[doc = "Field `PBE13` writer - 13:13\\]
PBE13 event"]
pub type Pbe13W<'a, REG> = crate::BitWriter<'a, REG, Pbe13>;
impl<'a, REG> Pbe13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe13::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe13::Noeff)
    }
}
#[doc = "14:14\\]
PBE14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe14 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe14> for bool {
    #[inline(always)]
    fn from(variant: Pbe14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE14` reader - 14:14\\]
PBE14 event"]
pub type Pbe14R = crate::BitReader<Pbe14>;
impl Pbe14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe14 {
        match self.bits {
            true => Pbe14::Set,
            false => Pbe14::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe14::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe14::Noeff
    }
}
#[doc = "Field `PBE14` writer - 14:14\\]
PBE14 event"]
pub type Pbe14W<'a, REG> = crate::BitWriter<'a, REG, Pbe14>;
impl<'a, REG> Pbe14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe14::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe14::Noeff)
    }
}
#[doc = "15:15\\]
PBE15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe15 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Pbe15> for bool {
    #[inline(always)]
    fn from(variant: Pbe15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE15` reader - 15:15\\]
PBE15 event"]
pub type Pbe15R = crate::BitReader<Pbe15>;
impl Pbe15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe15 {
        match self.bits {
            true => Pbe15::Set,
            false => Pbe15::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pbe15::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pbe15::Noeff
    }
}
#[doc = "Field `PBE15` writer - 15:15\\]
PBE15 event"]
pub type Pbe15W<'a, REG> = crate::BitWriter<'a, REG, Pbe15>;
impl<'a, REG> Pbe15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe15::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe15::Noeff)
    }
}
#[doc = "16:16\\]
RXFIFO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfifo {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Rxfifo> for bool {
    #[inline(always)]
    fn from(variant: Rxfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFO` reader - 16:16\\]
RXFIFO event"]
pub type RxfifoR = crate::BitReader<Rxfifo>;
impl RxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfifo {
        match self.bits {
            true => Rxfifo::Set,
            false => Rxfifo::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxfifo::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rxfifo::Noeff
    }
}
#[doc = "Field `RXFIFO` writer - 16:16\\]
RXFIFO event"]
pub type RxfifoW<'a, REG> = crate::BitWriter<'a, REG, Rxfifo>;
impl<'a, REG> RxfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifo::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifo::Noeff)
    }
}
#[doc = "17:17\\]
TXFIFO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfifo {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Txfifo> for bool {
    #[inline(always)]
    fn from(variant: Txfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFO` reader - 17:17\\]
TXFIFO event"]
pub type TxfifoR = crate::BitReader<Txfifo>;
impl TxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfifo {
        match self.bits {
            true => Txfifo::Set,
            false => Txfifo::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txfifo::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Txfifo::Noeff
    }
}
#[doc = "Field `TXFIFO` writer - 17:17\\]
TXFIFO event"]
pub type TxfifoW<'a, REG> = crate::BitWriter<'a, REG, Txfifo>;
impl<'a, REG> TxfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifo::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifo::Noeff)
    }
}
#[doc = "18:18\\]
LOSS_OF_LOCK event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lol {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Lol> for bool {
    #[inline(always)]
    fn from(variant: Lol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOL` reader - 18:18\\]
LOSS_OF_LOCK event"]
pub type LolR = crate::BitReader<Lol>;
impl LolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lol {
        match self.bits {
            true => Lol::Set,
            false => Lol::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Lol::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Lol::Noeff
    }
}
#[doc = "Field `LOL` writer - 18:18\\]
LOSS_OF_LOCK event"]
pub type LolW<'a, REG> = crate::BitWriter<'a, REG, Lol>;
impl<'a, REG> LolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Lol::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Lol::Noeff)
    }
}
#[doc = "19:19\\]
LOCK event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - 19:19\\]
LOCK event"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            true => Lock::Set,
            false => Lock::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Lock::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Lock::Noeff
    }
}
#[doc = "Field `LOCK` writer - 19:19\\]
LOCK event"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Noeff)
    }
}
#[doc = "20:20\\]
RFESOFT0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfesoft0 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Rfesoft0> for bool {
    #[inline(always)]
    fn from(variant: Rfesoft0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFESOFT0` reader - 20:20\\]
RFESOFT0 event"]
pub type Rfesoft0R = crate::BitReader<Rfesoft0>;
impl Rfesoft0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfesoft0 {
        match self.bits {
            true => Rfesoft0::Set,
            false => Rfesoft0::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rfesoft0::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rfesoft0::Noeff
    }
}
#[doc = "Field `RFESOFT0` writer - 20:20\\]
RFESOFT0 event"]
pub type Rfesoft0W<'a, REG> = crate::BitWriter<'a, REG, Rfesoft0>;
impl<'a, REG> Rfesoft0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft0::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft0::Noeff)
    }
}
#[doc = "21:21\\]
RFESOFT1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfesoft1 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Rfesoft1> for bool {
    #[inline(always)]
    fn from(variant: Rfesoft1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFESOFT1` reader - 21:21\\]
RFESOFT1 event"]
pub type Rfesoft1R = crate::BitReader<Rfesoft1>;
impl Rfesoft1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfesoft1 {
        match self.bits {
            true => Rfesoft1::Set,
            false => Rfesoft1::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rfesoft1::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rfesoft1::Noeff
    }
}
#[doc = "Field `RFESOFT1` writer - 21:21\\]
RFESOFT1 event"]
pub type Rfesoft1W<'a, REG> = crate::BitWriter<'a, REG, Rfesoft1>;
impl<'a, REG> Rfesoft1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft1::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft1::Noeff)
    }
}
#[doc = "22:22\\]
RFEDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfedone {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Rfedone> for bool {
    #[inline(always)]
    fn from(variant: Rfedone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEDONE` reader - 22:22\\]
RFEDONE event"]
pub type RfedoneR = crate::BitReader<Rfedone>;
impl RfedoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfedone {
        match self.bits {
            true => Rfedone::Set,
            false => Rfedone::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rfedone::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rfedone::Noeff
    }
}
#[doc = "Field `RFEDONE` writer - 22:22\\]
RFEDONE event"]
pub type RfedoneW<'a, REG> = crate::BitWriter<'a, REG, Rfedone>;
impl<'a, REG> RfedoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rfedone::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rfedone::Noeff)
    }
}
#[doc = "23:23\\]
MDMSOFT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmsoft0 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Mdmsoft0> for bool {
    #[inline(always)]
    fn from(variant: Mdmsoft0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMSOFT0` reader - 23:23\\]
MDMSOFT event"]
pub type Mdmsoft0R = crate::BitReader<Mdmsoft0>;
impl Mdmsoft0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmsoft0 {
        match self.bits {
            true => Mdmsoft0::Set,
            false => Mdmsoft0::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mdmsoft0::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Mdmsoft0::Noeff
    }
}
#[doc = "Field `MDMSOFT0` writer - 23:23\\]
MDMSOFT event"]
pub type Mdmsoft0W<'a, REG> = crate::BitWriter<'a, REG, Mdmsoft0>;
impl<'a, REG> Mdmsoft0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft0::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft0::Noeff)
    }
}
#[doc = "24:24\\]
MDMSOFT1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmsoft1 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Mdmsoft1> for bool {
    #[inline(always)]
    fn from(variant: Mdmsoft1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMSOFT1` reader - 24:24\\]
MDMSOFT1 event"]
pub type Mdmsoft1R = crate::BitReader<Mdmsoft1>;
impl Mdmsoft1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmsoft1 {
        match self.bits {
            true => Mdmsoft1::Set,
            false => Mdmsoft1::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mdmsoft1::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Mdmsoft1::Noeff
    }
}
#[doc = "Field `MDMSOFT1` writer - 24:24\\]
MDMSOFT1 event"]
pub type Mdmsoft1W<'a, REG> = crate::BitWriter<'a, REG, Mdmsoft1>;
impl<'a, REG> Mdmsoft1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft1::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft1::Noeff)
    }
}
#[doc = "25:25\\]
MDMSOFT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmsoft2 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Mdmsoft2> for bool {
    #[inline(always)]
    fn from(variant: Mdmsoft2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMSOFT2` reader - 25:25\\]
MDMSOFT event"]
pub type Mdmsoft2R = crate::BitReader<Mdmsoft2>;
impl Mdmsoft2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmsoft2 {
        match self.bits {
            true => Mdmsoft2::Set,
            false => Mdmsoft2::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mdmsoft2::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Mdmsoft2::Noeff
    }
}
#[doc = "Field `MDMSOFT2` writer - 25:25\\]
MDMSOFT event"]
pub type Mdmsoft2W<'a, REG> = crate::BitWriter<'a, REG, Mdmsoft2>;
impl<'a, REG> Mdmsoft2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft2::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft2::Noeff)
    }
}
#[doc = "26:26\\]
MDMOUT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmout {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Mdmout> for bool {
    #[inline(always)]
    fn from(variant: Mdmout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMOUT` reader - 26:26\\]
MDMOUT event"]
pub type MdmoutR = crate::BitReader<Mdmout>;
impl MdmoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmout {
        match self.bits {
            true => Mdmout::Set,
            false => Mdmout::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mdmout::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Mdmout::Noeff
    }
}
#[doc = "Field `MDMOUT` writer - 26:26\\]
MDMOUT event"]
pub type MdmoutW<'a, REG> = crate::BitWriter<'a, REG, Mdmout>;
impl<'a, REG> MdmoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmout::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmout::Noeff)
    }
}
#[doc = "27:27\\]
MDMIN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmin {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Mdmin> for bool {
    #[inline(always)]
    fn from(variant: Mdmin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMIN` reader - 27:27\\]
MDMIN event"]
pub type MdminR = crate::BitReader<Mdmin>;
impl MdminR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmin {
        match self.bits {
            true => Mdmin::Set,
            false => Mdmin::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mdmin::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Mdmin::Noeff
    }
}
#[doc = "Field `MDMIN` writer - 27:27\\]
MDMIN event"]
pub type MdminW<'a, REG> = crate::BitWriter<'a, REG, Mdmin>;
impl<'a, REG> MdminW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmin::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmin::Noeff)
    }
}
#[doc = "28:28\\]
MDMDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmdone {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Mdmdone> for bool {
    #[inline(always)]
    fn from(variant: Mdmdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMDONE` reader - 28:28\\]
MDMDONE event"]
pub type MdmdoneR = crate::BitReader<Mdmdone>;
impl MdmdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmdone {
        match self.bits {
            true => Mdmdone::Set,
            false => Mdmdone::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mdmdone::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Mdmdone::Noeff
    }
}
#[doc = "Field `MDMDONE` writer - 28:28\\]
MDMDONE event"]
pub type MdmdoneW<'a, REG> = crate::BitWriter<'a, REG, Mdmdone>;
impl<'a, REG> MdmdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmdone::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmdone::Noeff)
    }
}
#[doc = "29:29\\]
SYSTIM0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systim0 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Systim0> for bool {
    #[inline(always)]
    fn from(variant: Systim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTIM0` reader - 29:29\\]
SYSTIM0 event"]
pub type Systim0R = crate::BitReader<Systim0>;
impl Systim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systim0 {
        match self.bits {
            true => Systim0::Set,
            false => Systim0::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Systim0::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Systim0::Noeff
    }
}
#[doc = "Field `SYSTIM0` writer - 29:29\\]
SYSTIM0 event"]
pub type Systim0W<'a, REG> = crate::BitWriter<'a, REG, Systim0>;
impl<'a, REG> Systim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Systim0::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Systim0::Noeff)
    }
}
#[doc = "30:30\\]
SYSTIM1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systim1 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Systim1> for bool {
    #[inline(always)]
    fn from(variant: Systim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTIM1` reader - 30:30\\]
SYSTIM1 event"]
pub type Systim1R = crate::BitReader<Systim1>;
impl Systim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systim1 {
        match self.bits {
            true => Systim1::Set,
            false => Systim1::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Systim1::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Systim1::Noeff
    }
}
#[doc = "Field `SYSTIM1` writer - 30:30\\]
SYSTIM1 event"]
pub type Systim1W<'a, REG> = crate::BitWriter<'a, REG, Systim1>;
impl<'a, REG> Systim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Systim1::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Systim1::Noeff)
    }
}
#[doc = "31:31\\]
SYSTIM2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systim2 {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Systim2> for bool {
    #[inline(always)]
    fn from(variant: Systim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTIM2` reader - 31:31\\]
SYSTIM2 event"]
pub type Systim2R = crate::BitReader<Systim2>;
impl Systim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systim2 {
        match self.bits {
            true => Systim2::Set,
            false => Systim2::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Systim2::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Systim2::Noeff
    }
}
#[doc = "Field `SYSTIM2` writer - 31:31\\]
SYSTIM2 event"]
pub type Systim2W<'a, REG> = crate::BitWriter<'a, REG, Systim2>;
impl<'a, REG> Systim2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Systim2::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Systim2::Noeff)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
PBE0 event"]
    #[inline(always)]
    pub fn pbe0(&self) -> Pbe0R {
        Pbe0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PBE1 event"]
    #[inline(always)]
    pub fn pbe1(&self) -> Pbe1R {
        Pbe1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
PBE2 event"]
    #[inline(always)]
    pub fn pbe2(&self) -> Pbe2R {
        Pbe2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
PBE3 event"]
    #[inline(always)]
    pub fn pbe3(&self) -> Pbe3R {
        Pbe3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
PBE4 event"]
    #[inline(always)]
    pub fn pbe4(&self) -> Pbe4R {
        Pbe4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
PBE5 event"]
    #[inline(always)]
    pub fn pbe5(&self) -> Pbe5R {
        Pbe5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
PBE6 event"]
    #[inline(always)]
    pub fn pbe6(&self) -> Pbe6R {
        Pbe6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
PBE7 event"]
    #[inline(always)]
    pub fn pbe7(&self) -> Pbe7R {
        Pbe7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PBE8 event"]
    #[inline(always)]
    pub fn pbe8(&self) -> Pbe8R {
        Pbe8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
PBE9 event"]
    #[inline(always)]
    pub fn pbe9(&self) -> Pbe9R {
        Pbe9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
PBE10 event"]
    #[inline(always)]
    pub fn pbe10(&self) -> Pbe10R {
        Pbe10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
PBE11 event"]
    #[inline(always)]
    pub fn pbe11(&self) -> Pbe11R {
        Pbe11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
PBE12 event"]
    #[inline(always)]
    pub fn pbe12(&self) -> Pbe12R {
        Pbe12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
PBE13 event"]
    #[inline(always)]
    pub fn pbe13(&self) -> Pbe13R {
        Pbe13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
PBE14 event"]
    #[inline(always)]
    pub fn pbe14(&self) -> Pbe14R {
        Pbe14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
PBE15 event"]
    #[inline(always)]
    pub fn pbe15(&self) -> Pbe15R {
        Pbe15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RXFIFO event"]
    #[inline(always)]
    pub fn rxfifo(&self) -> RxfifoR {
        RxfifoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
TXFIFO event"]
    #[inline(always)]
    pub fn txfifo(&self) -> TxfifoR {
        TxfifoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
LOSS_OF_LOCK event"]
    #[inline(always)]
    pub fn lol(&self) -> LolR {
        LolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
LOCK event"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
RFESOFT0 event"]
    #[inline(always)]
    pub fn rfesoft0(&self) -> Rfesoft0R {
        Rfesoft0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
RFESOFT1 event"]
    #[inline(always)]
    pub fn rfesoft1(&self) -> Rfesoft1R {
        Rfesoft1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
RFEDONE event"]
    #[inline(always)]
    pub fn rfedone(&self) -> RfedoneR {
        RfedoneR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
MDMSOFT event"]
    #[inline(always)]
    pub fn mdmsoft0(&self) -> Mdmsoft0R {
        Mdmsoft0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
MDMSOFT1 event"]
    #[inline(always)]
    pub fn mdmsoft1(&self) -> Mdmsoft1R {
        Mdmsoft1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
MDMSOFT event"]
    #[inline(always)]
    pub fn mdmsoft2(&self) -> Mdmsoft2R {
        Mdmsoft2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
MDMOUT event"]
    #[inline(always)]
    pub fn mdmout(&self) -> MdmoutR {
        MdmoutR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
MDMIN event"]
    #[inline(always)]
    pub fn mdmin(&self) -> MdminR {
        MdminR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
MDMDONE event"]
    #[inline(always)]
    pub fn mdmdone(&self) -> MdmdoneR {
        MdmdoneR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
SYSTIM0 event"]
    #[inline(always)]
    pub fn systim0(&self) -> Systim0R {
        Systim0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
SYSTIM1 event"]
    #[inline(always)]
    pub fn systim1(&self) -> Systim1R {
        Systim1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
SYSTIM2 event"]
    #[inline(always)]
    pub fn systim2(&self) -> Systim2R {
        Systim2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PBE0 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe0(&mut self) -> Pbe0W<Iset2Spec> {
        Pbe0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PBE1 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe1(&mut self) -> Pbe1W<Iset2Spec> {
        Pbe1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
PBE2 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe2(&mut self) -> Pbe2W<Iset2Spec> {
        Pbe2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
PBE3 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe3(&mut self) -> Pbe3W<Iset2Spec> {
        Pbe3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
PBE4 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe4(&mut self) -> Pbe4W<Iset2Spec> {
        Pbe4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
PBE5 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe5(&mut self) -> Pbe5W<Iset2Spec> {
        Pbe5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
PBE6 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe6(&mut self) -> Pbe6W<Iset2Spec> {
        Pbe6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
PBE7 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe7(&mut self) -> Pbe7W<Iset2Spec> {
        Pbe7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
PBE8 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe8(&mut self) -> Pbe8W<Iset2Spec> {
        Pbe8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
PBE9 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe9(&mut self) -> Pbe9W<Iset2Spec> {
        Pbe9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
PBE10 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe10(&mut self) -> Pbe10W<Iset2Spec> {
        Pbe10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
PBE11 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe11(&mut self) -> Pbe11W<Iset2Spec> {
        Pbe11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
PBE12 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe12(&mut self) -> Pbe12W<Iset2Spec> {
        Pbe12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
PBE13 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe13(&mut self) -> Pbe13W<Iset2Spec> {
        Pbe13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
PBE14 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe14(&mut self) -> Pbe14W<Iset2Spec> {
        Pbe14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
PBE15 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe15(&mut self) -> Pbe15W<Iset2Spec> {
        Pbe15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
RXFIFO event"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo(&mut self) -> RxfifoW<Iset2Spec> {
        RxfifoW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
TXFIFO event"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo(&mut self) -> TxfifoW<Iset2Spec> {
        TxfifoW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
LOSS_OF_LOCK event"]
    #[inline(always)]
    #[must_use]
    pub fn lol(&mut self) -> LolW<Iset2Spec> {
        LolW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
LOCK event"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<Iset2Spec> {
        LockW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
RFESOFT0 event"]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft0(&mut self) -> Rfesoft0W<Iset2Spec> {
        Rfesoft0W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
RFESOFT1 event"]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft1(&mut self) -> Rfesoft1W<Iset2Spec> {
        Rfesoft1W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
RFEDONE event"]
    #[inline(always)]
    #[must_use]
    pub fn rfedone(&mut self) -> RfedoneW<Iset2Spec> {
        RfedoneW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
MDMSOFT event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft0(&mut self) -> Mdmsoft0W<Iset2Spec> {
        Mdmsoft0W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
MDMSOFT1 event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft1(&mut self) -> Mdmsoft1W<Iset2Spec> {
        Mdmsoft1W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
MDMSOFT event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft2(&mut self) -> Mdmsoft2W<Iset2Spec> {
        Mdmsoft2W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
MDMOUT event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmout(&mut self) -> MdmoutW<Iset2Spec> {
        MdmoutW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
MDMIN event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmin(&mut self) -> MdminW<Iset2Spec> {
        MdminW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
MDMDONE event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmdone(&mut self) -> MdmdoneW<Iset2Spec> {
        MdmdoneW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
SYSTIM0 event"]
    #[inline(always)]
    #[must_use]
    pub fn systim0(&mut self) -> Systim0W<Iset2Spec> {
        Systim0W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
SYSTIM1 event"]
    #[inline(always)]
    #[must_use]
    pub fn systim1(&mut self) -> Systim1W<Iset2Spec> {
        Systim1W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
SYSTIM2 event"]
    #[inline(always)]
    #[must_use]
    pub fn systim2(&mut self) -> Systim2W<Iset2Spec> {
        Systim2W::new(self, 31)
    }
}
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iset2Spec;
impl crate::RegisterSpec for Iset2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iset2::R`](R) reader structure"]
impl crate::Readable for Iset2Spec {}
#[doc = "`write(|w| ..)` method takes [`iset2::W`](W) writer structure"]
impl crate::Writable for Iset2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET2 to value 0"]
impl crate::Resettable for Iset2Spec {
    const RESET_VALUE: u32 = 0;
}