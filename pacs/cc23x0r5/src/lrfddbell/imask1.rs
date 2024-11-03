#[doc = "Register `IMASK1` reader"]
pub type R = crate::R<Imask1Spec>;
#[doc = "Register `IMASK1` writer"]
pub type W = crate::W<Imask1Spec>;
#[doc = "0:0\\]
PBE0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe0 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe0::En,
            false => Pbe0::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe0::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe0::Dis
    }
}
#[doc = "Field `PBE0` writer - 0:0\\]
PBE0 event"]
pub type Pbe0W<'a, REG> = crate::BitWriter<'a, REG, Pbe0>;
impl<'a, REG> Pbe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe0::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe0::Dis)
    }
}
#[doc = "1:1\\]
PBE1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe1 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe1::En,
            false => Pbe1::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe1::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe1::Dis
    }
}
#[doc = "Field `PBE1` writer - 1:1\\]
PBE1 event"]
pub type Pbe1W<'a, REG> = crate::BitWriter<'a, REG, Pbe1>;
impl<'a, REG> Pbe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe1::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe1::Dis)
    }
}
#[doc = "2:2\\]
PBE2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe2 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe2::En,
            false => Pbe2::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe2::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe2::Dis
    }
}
#[doc = "Field `PBE2` writer - 2:2\\]
PBE2 event"]
pub type Pbe2W<'a, REG> = crate::BitWriter<'a, REG, Pbe2>;
impl<'a, REG> Pbe2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe2::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe2::Dis)
    }
}
#[doc = "3:3\\]
PBE3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe3 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe3::En,
            false => Pbe3::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe3::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe3::Dis
    }
}
#[doc = "Field `PBE3` writer - 3:3\\]
PBE3 event"]
pub type Pbe3W<'a, REG> = crate::BitWriter<'a, REG, Pbe3>;
impl<'a, REG> Pbe3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe3::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe3::Dis)
    }
}
#[doc = "4:4\\]
PBE4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe4 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe4::En,
            false => Pbe4::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe4::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe4::Dis
    }
}
#[doc = "Field `PBE4` writer - 4:4\\]
PBE4 event"]
pub type Pbe4W<'a, REG> = crate::BitWriter<'a, REG, Pbe4>;
impl<'a, REG> Pbe4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe4::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe4::Dis)
    }
}
#[doc = "5:5\\]
PBE5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe5 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe5::En,
            false => Pbe5::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe5::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe5::Dis
    }
}
#[doc = "Field `PBE5` writer - 5:5\\]
PBE5 event"]
pub type Pbe5W<'a, REG> = crate::BitWriter<'a, REG, Pbe5>;
impl<'a, REG> Pbe5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe5::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe5::Dis)
    }
}
#[doc = "6:6\\]
PBE6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe6 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe6::En,
            false => Pbe6::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe6::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe6::Dis
    }
}
#[doc = "Field `PBE6` writer - 6:6\\]
PBE6 event"]
pub type Pbe6W<'a, REG> = crate::BitWriter<'a, REG, Pbe6>;
impl<'a, REG> Pbe6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe6::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe6::Dis)
    }
}
#[doc = "7:7\\]
PBE7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe7 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe7::En,
            false => Pbe7::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe7::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe7::Dis
    }
}
#[doc = "Field `PBE7` writer - 7:7\\]
PBE7 event"]
pub type Pbe7W<'a, REG> = crate::BitWriter<'a, REG, Pbe7>;
impl<'a, REG> Pbe7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe7::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe7::Dis)
    }
}
#[doc = "8:8\\]
PBE8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe8 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
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
            true => Pbe8::En,
            false => Pbe8::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe8::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe8::Dis
    }
}
#[doc = "Field `PBE8` writer - 8:8\\]
PBE8 event"]
pub type Pbe8W<'a, REG> = crate::BitWriter<'a, REG, Pbe8>;
impl<'a, REG> Pbe8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe8::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe8::Dis)
    }
}
#[doc = "9:9\\]
PBE10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe10 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Pbe10> for bool {
    #[inline(always)]
    fn from(variant: Pbe10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE10` reader - 9:9\\]
PBE10 event"]
pub type Pbe10R = crate::BitReader<Pbe10>;
impl Pbe10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe10 {
        match self.bits {
            true => Pbe10::En,
            false => Pbe10::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe10::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe10::Dis
    }
}
#[doc = "Field `PBE10` writer - 9:9\\]
PBE10 event"]
pub type Pbe10W<'a, REG> = crate::BitWriter<'a, REG, Pbe10>;
impl<'a, REG> Pbe10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe10::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe10::Dis)
    }
}
#[doc = "10:10\\]
PBE11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe11 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Pbe11> for bool {
    #[inline(always)]
    fn from(variant: Pbe11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE11` reader - 10:10\\]
PBE11 event"]
pub type Pbe11R = crate::BitReader<Pbe11>;
impl Pbe11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe11 {
        match self.bits {
            true => Pbe11::En,
            false => Pbe11::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe11::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe11::Dis
    }
}
#[doc = "Field `PBE11` writer - 10:10\\]
PBE11 event"]
pub type Pbe11W<'a, REG> = crate::BitWriter<'a, REG, Pbe11>;
impl<'a, REG> Pbe11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe11::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe11::Dis)
    }
}
#[doc = "11:11\\]
PBE12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe12 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Pbe12> for bool {
    #[inline(always)]
    fn from(variant: Pbe12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE12` reader - 11:11\\]
PBE12 event"]
pub type Pbe12R = crate::BitReader<Pbe12>;
impl Pbe12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe12 {
        match self.bits {
            true => Pbe12::En,
            false => Pbe12::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe12::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe12::Dis
    }
}
#[doc = "Field `PBE12` writer - 11:11\\]
PBE12 event"]
pub type Pbe12W<'a, REG> = crate::BitWriter<'a, REG, Pbe12>;
impl<'a, REG> Pbe12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe12::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe12::Dis)
    }
}
#[doc = "12:12\\]
PBE13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe13 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Pbe13> for bool {
    #[inline(always)]
    fn from(variant: Pbe13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE13` reader - 12:12\\]
PBE13 event"]
pub type Pbe13R = crate::BitReader<Pbe13>;
impl Pbe13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe13 {
        match self.bits {
            true => Pbe13::En,
            false => Pbe13::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe13::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe13::Dis
    }
}
#[doc = "Field `PBE13` writer - 12:12\\]
PBE13 event"]
pub type Pbe13W<'a, REG> = crate::BitWriter<'a, REG, Pbe13>;
impl<'a, REG> Pbe13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe13::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe13::Dis)
    }
}
#[doc = "13:13\\]
PBE14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe14 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Pbe14> for bool {
    #[inline(always)]
    fn from(variant: Pbe14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE14` reader - 13:13\\]
PBE14 event"]
pub type Pbe14R = crate::BitReader<Pbe14>;
impl Pbe14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe14 {
        match self.bits {
            true => Pbe14::En,
            false => Pbe14::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe14::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe14::Dis
    }
}
#[doc = "Field `PBE14` writer - 13:13\\]
PBE14 event"]
pub type Pbe14W<'a, REG> = crate::BitWriter<'a, REG, Pbe14>;
impl<'a, REG> Pbe14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe14::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe14::Dis)
    }
}
#[doc = "14:14\\]
PBE15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe15 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Pbe15> for bool {
    #[inline(always)]
    fn from(variant: Pbe15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE15` reader - 14:14\\]
PBE15 event"]
pub type Pbe15R = crate::BitReader<Pbe15>;
impl Pbe15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe15 {
        match self.bits {
            true => Pbe15::En,
            false => Pbe15::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe15::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe15::Dis
    }
}
#[doc = "Field `PBE15` writer - 14:14\\]
PBE15 event"]
pub type Pbe15W<'a, REG> = crate::BitWriter<'a, REG, Pbe15>;
impl<'a, REG> Pbe15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe15::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe15::Dis)
    }
}
#[doc = "15:15\\]
RXFIFO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfifo {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Rxfifo> for bool {
    #[inline(always)]
    fn from(variant: Rxfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFO` reader - 15:15\\]
RXFIFO event"]
pub type RxfifoR = crate::BitReader<Rxfifo>;
impl RxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfifo {
        match self.bits {
            true => Rxfifo::En,
            false => Rxfifo::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rxfifo::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rxfifo::Dis
    }
}
#[doc = "Field `RXFIFO` writer - 15:15\\]
RXFIFO event"]
pub type RxfifoW<'a, REG> = crate::BitWriter<'a, REG, Rxfifo>;
impl<'a, REG> RxfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifo::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifo::Dis)
    }
}
#[doc = "16:16\\]
TXFIFO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfifo {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Txfifo> for bool {
    #[inline(always)]
    fn from(variant: Txfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFO` reader - 16:16\\]
TXFIFO event"]
pub type TxfifoR = crate::BitReader<Txfifo>;
impl TxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfifo {
        match self.bits {
            true => Txfifo::En,
            false => Txfifo::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Txfifo::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Txfifo::Dis
    }
}
#[doc = "Field `TXFIFO` writer - 16:16\\]
TXFIFO event"]
pub type TxfifoW<'a, REG> = crate::BitWriter<'a, REG, Txfifo>;
impl<'a, REG> TxfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifo::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifo::Dis)
    }
}
#[doc = "17:17\\]
LOSS_OF_LOCK event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lol {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Lol> for bool {
    #[inline(always)]
    fn from(variant: Lol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOL` reader - 17:17\\]
LOSS_OF_LOCK event"]
pub type LolR = crate::BitReader<Lol>;
impl LolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lol {
        match self.bits {
            true => Lol::En,
            false => Lol::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Lol::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lol::Dis
    }
}
#[doc = "Field `LOL` writer - 17:17\\]
LOSS_OF_LOCK event"]
pub type LolW<'a, REG> = crate::BitWriter<'a, REG, Lol>;
impl<'a, REG> LolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Lol::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lol::Dis)
    }
}
#[doc = "18:18\\]
LOCK event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - 18:18\\]
LOCK event"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            true => Lock::En,
            false => Lock::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Lock::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lock::Dis
    }
}
#[doc = "Field `LOCK` writer - 18:18\\]
LOCK event"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Dis)
    }
}
#[doc = "19:19\\]
RFESOFT0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfesoft0 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Rfesoft0> for bool {
    #[inline(always)]
    fn from(variant: Rfesoft0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFESOFT0` reader - 19:19\\]
RFESOFT0 event"]
pub type Rfesoft0R = crate::BitReader<Rfesoft0>;
impl Rfesoft0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfesoft0 {
        match self.bits {
            true => Rfesoft0::En,
            false => Rfesoft0::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rfesoft0::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rfesoft0::Dis
    }
}
#[doc = "Field `RFESOFT0` writer - 19:19\\]
RFESOFT0 event"]
pub type Rfesoft0W<'a, REG> = crate::BitWriter<'a, REG, Rfesoft0>;
impl<'a, REG> Rfesoft0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft0::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft0::Dis)
    }
}
#[doc = "20:20\\]
RFESOFT1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfesoft1 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Rfesoft1> for bool {
    #[inline(always)]
    fn from(variant: Rfesoft1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFESOFT1` reader - 20:20\\]
RFESOFT1 event"]
pub type Rfesoft1R = crate::BitReader<Rfesoft1>;
impl Rfesoft1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfesoft1 {
        match self.bits {
            true => Rfesoft1::En,
            false => Rfesoft1::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rfesoft1::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rfesoft1::Dis
    }
}
#[doc = "Field `RFESOFT1` writer - 20:20\\]
RFESOFT1 event"]
pub type Rfesoft1W<'a, REG> = crate::BitWriter<'a, REG, Rfesoft1>;
impl<'a, REG> Rfesoft1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft1::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rfesoft1::Dis)
    }
}
#[doc = "21:21\\]
RFEDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfedone {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Rfedone> for bool {
    #[inline(always)]
    fn from(variant: Rfedone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEDONE` reader - 21:21\\]
RFEDONE event"]
pub type RfedoneR = crate::BitReader<Rfedone>;
impl RfedoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfedone {
        match self.bits {
            true => Rfedone::En,
            false => Rfedone::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rfedone::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rfedone::Dis
    }
}
#[doc = "Field `RFEDONE` writer - 21:21\\]
RFEDONE event"]
pub type RfedoneW<'a, REG> = crate::BitWriter<'a, REG, Rfedone>;
impl<'a, REG> RfedoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rfedone::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rfedone::Dis)
    }
}
#[doc = "22:22\\]
MDMSOFT2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmsoft0 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Mdmsoft0> for bool {
    #[inline(always)]
    fn from(variant: Mdmsoft0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMSOFT0` reader - 22:22\\]
MDMSOFT2 event"]
pub type Mdmsoft0R = crate::BitReader<Mdmsoft0>;
impl Mdmsoft0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmsoft0 {
        match self.bits {
            true => Mdmsoft0::En,
            false => Mdmsoft0::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mdmsoft0::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mdmsoft0::Dis
    }
}
#[doc = "Field `MDMSOFT0` writer - 22:22\\]
MDMSOFT2 event"]
pub type Mdmsoft0W<'a, REG> = crate::BitWriter<'a, REG, Mdmsoft0>;
impl<'a, REG> Mdmsoft0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft0::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft0::Dis)
    }
}
#[doc = "23:23\\]
MDMSOFT2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmsoft1 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Mdmsoft1> for bool {
    #[inline(always)]
    fn from(variant: Mdmsoft1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMSOFT1` reader - 23:23\\]
MDMSOFT2 event"]
pub type Mdmsoft1R = crate::BitReader<Mdmsoft1>;
impl Mdmsoft1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmsoft1 {
        match self.bits {
            true => Mdmsoft1::En,
            false => Mdmsoft1::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mdmsoft1::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mdmsoft1::Dis
    }
}
#[doc = "Field `MDMSOFT1` writer - 23:23\\]
MDMSOFT2 event"]
pub type Mdmsoft1W<'a, REG> = crate::BitWriter<'a, REG, Mdmsoft1>;
impl<'a, REG> Mdmsoft1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft1::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft1::Dis)
    }
}
#[doc = "24:24\\]
MDMSOFT2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmsoft2 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Mdmsoft2> for bool {
    #[inline(always)]
    fn from(variant: Mdmsoft2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMSOFT2` reader - 24:24\\]
MDMSOFT2 event"]
pub type Mdmsoft2R = crate::BitReader<Mdmsoft2>;
impl Mdmsoft2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmsoft2 {
        match self.bits {
            true => Mdmsoft2::En,
            false => Mdmsoft2::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mdmsoft2::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mdmsoft2::Dis
    }
}
#[doc = "Field `MDMSOFT2` writer - 24:24\\]
MDMSOFT2 event"]
pub type Mdmsoft2W<'a, REG> = crate::BitWriter<'a, REG, Mdmsoft2>;
impl<'a, REG> Mdmsoft2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft2::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmsoft2::Dis)
    }
}
#[doc = "25:25\\]
MDMOUT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmout {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Mdmout> for bool {
    #[inline(always)]
    fn from(variant: Mdmout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMOUT` reader - 25:25\\]
MDMOUT event"]
pub type MdmoutR = crate::BitReader<Mdmout>;
impl MdmoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmout {
        match self.bits {
            true => Mdmout::En,
            false => Mdmout::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mdmout::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mdmout::Dis
    }
}
#[doc = "Field `MDMOUT` writer - 25:25\\]
MDMOUT event"]
pub type MdmoutW<'a, REG> = crate::BitWriter<'a, REG, Mdmout>;
impl<'a, REG> MdmoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmout::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmout::Dis)
    }
}
#[doc = "26:26\\]
MDMIN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmin {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Mdmin> for bool {
    #[inline(always)]
    fn from(variant: Mdmin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMIN` reader - 26:26\\]
MDMIN event"]
pub type MdminR = crate::BitReader<Mdmin>;
impl MdminR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmin {
        match self.bits {
            true => Mdmin::En,
            false => Mdmin::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mdmin::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mdmin::Dis
    }
}
#[doc = "Field `MDMIN` writer - 26:26\\]
MDMIN event"]
pub type MdminW<'a, REG> = crate::BitWriter<'a, REG, Mdmin>;
impl<'a, REG> MdminW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmin::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmin::Dis)
    }
}
#[doc = "27:27\\]
MDMDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdmdone {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Mdmdone> for bool {
    #[inline(always)]
    fn from(variant: Mdmdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMDONE` reader - 27:27\\]
MDMDONE event"]
pub type MdmdoneR = crate::BitReader<Mdmdone>;
impl MdmdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdmdone {
        match self.bits {
            true => Mdmdone::En,
            false => Mdmdone::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mdmdone::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mdmdone::Dis
    }
}
#[doc = "Field `MDMDONE` writer - 27:27\\]
MDMDONE event"]
pub type MdmdoneW<'a, REG> = crate::BitWriter<'a, REG, Mdmdone>;
impl<'a, REG> MdmdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmdone::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mdmdone::Dis)
    }
}
#[doc = "28:28\\]
SYSTIM0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systim0 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Systim0> for bool {
    #[inline(always)]
    fn from(variant: Systim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTIM0` reader - 28:28\\]
SYSTIM0 event"]
pub type Systim0R = crate::BitReader<Systim0>;
impl Systim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systim0 {
        match self.bits {
            true => Systim0::En,
            false => Systim0::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Systim0::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Systim0::Dis
    }
}
#[doc = "Field `SYSTIM0` writer - 28:28\\]
SYSTIM0 event"]
pub type Systim0W<'a, REG> = crate::BitWriter<'a, REG, Systim0>;
impl<'a, REG> Systim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Systim0::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Systim0::Dis)
    }
}
#[doc = "29:29\\]
SYSTIM1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systim1 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Systim1> for bool {
    #[inline(always)]
    fn from(variant: Systim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTIM1` reader - 29:29\\]
SYSTIM1 event"]
pub type Systim1R = crate::BitReader<Systim1>;
impl Systim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systim1 {
        match self.bits {
            true => Systim1::En,
            false => Systim1::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Systim1::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Systim1::Dis
    }
}
#[doc = "Field `SYSTIM1` writer - 29:29\\]
SYSTIM1 event"]
pub type Systim1W<'a, REG> = crate::BitWriter<'a, REG, Systim1>;
impl<'a, REG> Systim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Systim1::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Systim1::Dis)
    }
}
#[doc = "30:30\\]
SYSTIM2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systim2 {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Systim2> for bool {
    #[inline(always)]
    fn from(variant: Systim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTIM2` reader - 30:30\\]
SYSTIM2 event"]
pub type Systim2R = crate::BitReader<Systim2>;
impl Systim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systim2 {
        match self.bits {
            true => Systim2::En,
            false => Systim2::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Systim2::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Systim2::Dis
    }
}
#[doc = "Field `SYSTIM2` writer - 30:30\\]
SYSTIM2 event"]
pub type Systim2W<'a, REG> = crate::BitWriter<'a, REG, Systim2>;
impl<'a, REG> Systim2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Systim2::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Systim2::Dis)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
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
PBE10 event"]
    #[inline(always)]
    pub fn pbe10(&self) -> Pbe10R {
        Pbe10R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
PBE11 event"]
    #[inline(always)]
    pub fn pbe11(&self) -> Pbe11R {
        Pbe11R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
PBE12 event"]
    #[inline(always)]
    pub fn pbe12(&self) -> Pbe12R {
        Pbe12R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
PBE13 event"]
    #[inline(always)]
    pub fn pbe13(&self) -> Pbe13R {
        Pbe13R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
PBE14 event"]
    #[inline(always)]
    pub fn pbe14(&self) -> Pbe14R {
        Pbe14R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
PBE15 event"]
    #[inline(always)]
    pub fn pbe15(&self) -> Pbe15R {
        Pbe15R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
RXFIFO event"]
    #[inline(always)]
    pub fn rxfifo(&self) -> RxfifoR {
        RxfifoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
TXFIFO event"]
    #[inline(always)]
    pub fn txfifo(&self) -> TxfifoR {
        TxfifoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
LOSS_OF_LOCK event"]
    #[inline(always)]
    pub fn lol(&self) -> LolR {
        LolR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
LOCK event"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
RFESOFT0 event"]
    #[inline(always)]
    pub fn rfesoft0(&self) -> Rfesoft0R {
        Rfesoft0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
RFESOFT1 event"]
    #[inline(always)]
    pub fn rfesoft1(&self) -> Rfesoft1R {
        Rfesoft1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
RFEDONE event"]
    #[inline(always)]
    pub fn rfedone(&self) -> RfedoneR {
        RfedoneR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
MDMSOFT2 event"]
    #[inline(always)]
    pub fn mdmsoft0(&self) -> Mdmsoft0R {
        Mdmsoft0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
MDMSOFT2 event"]
    #[inline(always)]
    pub fn mdmsoft1(&self) -> Mdmsoft1R {
        Mdmsoft1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
MDMSOFT2 event"]
    #[inline(always)]
    pub fn mdmsoft2(&self) -> Mdmsoft2R {
        Mdmsoft2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
MDMOUT event"]
    #[inline(always)]
    pub fn mdmout(&self) -> MdmoutR {
        MdmoutR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
MDMIN event"]
    #[inline(always)]
    pub fn mdmin(&self) -> MdminR {
        MdminR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
MDMDONE event"]
    #[inline(always)]
    pub fn mdmdone(&self) -> MdmdoneR {
        MdmdoneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
SYSTIM0 event"]
    #[inline(always)]
    pub fn systim0(&self) -> Systim0R {
        Systim0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
SYSTIM1 event"]
    #[inline(always)]
    pub fn systim1(&self) -> Systim1R {
        Systim1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
SYSTIM2 event"]
    #[inline(always)]
    pub fn systim2(&self) -> Systim2R {
        Systim2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PBE0 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe0(&mut self) -> Pbe0W<Imask1Spec> {
        Pbe0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PBE1 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe1(&mut self) -> Pbe1W<Imask1Spec> {
        Pbe1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
PBE2 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe2(&mut self) -> Pbe2W<Imask1Spec> {
        Pbe2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
PBE3 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe3(&mut self) -> Pbe3W<Imask1Spec> {
        Pbe3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
PBE4 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe4(&mut self) -> Pbe4W<Imask1Spec> {
        Pbe4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
PBE5 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe5(&mut self) -> Pbe5W<Imask1Spec> {
        Pbe5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
PBE6 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe6(&mut self) -> Pbe6W<Imask1Spec> {
        Pbe6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
PBE7 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe7(&mut self) -> Pbe7W<Imask1Spec> {
        Pbe7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
PBE8 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe8(&mut self) -> Pbe8W<Imask1Spec> {
        Pbe8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
PBE10 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe10(&mut self) -> Pbe10W<Imask1Spec> {
        Pbe10W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
PBE11 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe11(&mut self) -> Pbe11W<Imask1Spec> {
        Pbe11W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
PBE12 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe12(&mut self) -> Pbe12W<Imask1Spec> {
        Pbe12W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
PBE13 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe13(&mut self) -> Pbe13W<Imask1Spec> {
        Pbe13W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
PBE14 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe14(&mut self) -> Pbe14W<Imask1Spec> {
        Pbe14W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
PBE15 event"]
    #[inline(always)]
    #[must_use]
    pub fn pbe15(&mut self) -> Pbe15W<Imask1Spec> {
        Pbe15W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
RXFIFO event"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo(&mut self) -> RxfifoW<Imask1Spec> {
        RxfifoW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
TXFIFO event"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo(&mut self) -> TxfifoW<Imask1Spec> {
        TxfifoW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
LOSS_OF_LOCK event"]
    #[inline(always)]
    #[must_use]
    pub fn lol(&mut self) -> LolW<Imask1Spec> {
        LolW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
LOCK event"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<Imask1Spec> {
        LockW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
RFESOFT0 event"]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft0(&mut self) -> Rfesoft0W<Imask1Spec> {
        Rfesoft0W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
RFESOFT1 event"]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft1(&mut self) -> Rfesoft1W<Imask1Spec> {
        Rfesoft1W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
RFEDONE event"]
    #[inline(always)]
    #[must_use]
    pub fn rfedone(&mut self) -> RfedoneW<Imask1Spec> {
        RfedoneW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
MDMSOFT2 event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft0(&mut self) -> Mdmsoft0W<Imask1Spec> {
        Mdmsoft0W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
MDMSOFT2 event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft1(&mut self) -> Mdmsoft1W<Imask1Spec> {
        Mdmsoft1W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
MDMSOFT2 event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft2(&mut self) -> Mdmsoft2W<Imask1Spec> {
        Mdmsoft2W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
MDMOUT event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmout(&mut self) -> MdmoutW<Imask1Spec> {
        MdmoutW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
MDMIN event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmin(&mut self) -> MdminW<Imask1Spec> {
        MdminW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
MDMDONE event"]
    #[inline(always)]
    #[must_use]
    pub fn mdmdone(&mut self) -> MdmdoneW<Imask1Spec> {
        MdmdoneW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
SYSTIM0 event"]
    #[inline(always)]
    #[must_use]
    pub fn systim0(&mut self) -> Systim0W<Imask1Spec> {
        Systim0W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
SYSTIM1 event"]
    #[inline(always)]
    #[must_use]
    pub fn systim1(&mut self) -> Systim1W<Imask1Spec> {
        Systim1W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
SYSTIM2 event"]
    #[inline(always)]
    #[must_use]
    pub fn systim2(&mut self) -> Systim2W<Imask1Spec> {
        Systim2W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<Imask1Spec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imask1Spec;
impl crate::RegisterSpec for Imask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask1::R`](R) reader structure"]
impl crate::Readable for Imask1Spec {}
#[doc = "`write(|w| ..)` method takes [`imask1::W`](W) writer structure"]
impl crate::Writable for Imask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK1 to value 0"]
impl crate::Resettable for Imask1Spec {
    const RESET_VALUE: u32 = 0;
}
