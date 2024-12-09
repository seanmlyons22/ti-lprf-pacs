#[doc = "Register `ATESTCFG` reader"]
pub type R = crate::R<AtestcfgSpec>;
#[doc = "Register `ATESTCFG` writer"]
pub type W = crate::W<AtestcfgSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtvr0 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Shtvr0> for bool {
    #[inline(always)]
    fn from(variant: Shtvr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTVR0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Shtvr0R = crate::BitReader<Shtvr0>;
impl Shtvr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtvr0 {
        match self.bits {
            true => Shtvr0::Close,
            false => Shtvr0::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Shtvr0::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Shtvr0::Open
    }
}
#[doc = "Field `SHTVR0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Shtvr0W<'a, REG> = crate::BitWriter<'a, REG, Shtvr0>;
impl<'a, REG> Shtvr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Shtvr0::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Shtvr0::Open)
    }
}
#[doc = "1:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtvr1 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Shtvr1> for bool {
    #[inline(always)]
    fn from(variant: Shtvr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTVR1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Shtvr1R = crate::BitReader<Shtvr1>;
impl Shtvr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtvr1 {
        match self.bits {
            true => Shtvr1::Close,
            false => Shtvr1::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Shtvr1::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Shtvr1::Open
    }
}
#[doc = "Field `SHTVR1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Shtvr1W<'a, REG> = crate::BitWriter<'a, REG, Shtvr1>;
impl<'a, REG> Shtvr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Shtvr1::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Shtvr1::Open)
    }
}
#[doc = "2:2\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtva0 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Shtva0> for bool {
    #[inline(always)]
    fn from(variant: Shtva0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTVA0` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type Shtva0R = crate::BitReader<Shtva0>;
impl Shtva0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtva0 {
        match self.bits {
            true => Shtva0::Close,
            false => Shtva0::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Shtva0::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Shtva0::Open
    }
}
#[doc = "Field `SHTVA0` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type Shtva0W<'a, REG> = crate::BitWriter<'a, REG, Shtva0>;
impl<'a, REG> Shtva0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Shtva0::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Shtva0::Open)
    }
}
#[doc = "3:3\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtva1 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Shtva1> for bool {
    #[inline(always)]
    fn from(variant: Shtva1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTVA1` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type Shtva1R = crate::BitReader<Shtva1>;
impl Shtva1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtva1 {
        match self.bits {
            true => Shtva1::Close,
            false => Shtva1::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Shtva1::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Shtva1::Open
    }
}
#[doc = "Field `SHTVA1` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type Shtva1W<'a, REG> = crate::BitWriter<'a, REG, Shtva1>;
impl<'a, REG> Shtva1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Shtva1::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Shtva1::Open)
    }
}
#[doc = "4:4\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vr2va0 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Vr2va0> for bool {
    #[inline(always)]
    fn from(variant: Vr2va0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VR2VA0` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type Vr2va0R = crate::BitReader<Vr2va0>;
impl Vr2va0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vr2va0 {
        match self.bits {
            true => Vr2va0::Close,
            false => Vr2va0::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Vr2va0::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Vr2va0::Open
    }
}
#[doc = "Field `VR2VA0` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type Vr2va0W<'a, REG> = crate::BitWriter<'a, REG, Vr2va0>;
impl<'a, REG> Vr2va0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Vr2va0::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Vr2va0::Open)
    }
}
#[doc = "5:5\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vr2va1 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Vr2va1> for bool {
    #[inline(always)]
    fn from(variant: Vr2va1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VR2VA1` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type Vr2va1R = crate::BitReader<Vr2va1>;
impl Vr2va1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vr2va1 {
        match self.bits {
            true => Vr2va1::Close,
            false => Vr2va1::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Vr2va1::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Vr2va1::Open
    }
}
#[doc = "Field `VR2VA1` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type Vr2va1W<'a, REG> = crate::BitWriter<'a, REG, Vr2va1>;
impl<'a, REG> Vr2va1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Vr2va1::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Vr2va1::Open)
    }
}
#[doc = "6:6\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Va2va0 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Va2va0> for bool {
    #[inline(always)]
    fn from(variant: Va2va0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VA2VA0` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type Va2va0R = crate::BitReader<Va2va0>;
impl Va2va0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Va2va0 {
        match self.bits {
            true => Va2va0::Close,
            false => Va2va0::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Va2va0::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Va2va0::Open
    }
}
#[doc = "Field `VA2VA0` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type Va2va0W<'a, REG> = crate::BitWriter<'a, REG, Va2va0>;
impl<'a, REG> Va2va0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Va2va0::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Va2va0::Open)
    }
}
#[doc = "7:7\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Va2va1 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Close = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Va2va1> for bool {
    #[inline(always)]
    fn from(variant: Va2va1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VA2VA1` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type Va2va1R = crate::BitReader<Va2va1>;
impl Va2va1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Va2va1 {
        match self.bits {
            true => Va2va1::Close,
            false => Va2va1::Open,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        *self == Va2va1::Close
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Va2va1::Open
    }
}
#[doc = "Field `VA2VA1` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type Va2va1W<'a, REG> = crate::BitWriter<'a, REG, Va2va1>;
impl<'a, REG> Va2va1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn close(self) -> &'a mut crate::W<REG> {
        self.variant(Va2va1::Close)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Va2va1::Open)
    }
}
#[doc = "8:8\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsel {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Vdda = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Vddbst = 0,
}
impl From<Vsel> for bool {
    #[inline(always)]
    fn from(variant: Vsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSEL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type VselR = crate::BitReader<Vsel>;
impl VselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsel {
        match self.bits {
            true => Vsel::Vdda,
            false => Vsel::Vddbst,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == Vsel::Vdda
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vddbst(&self) -> bool {
        *self == Vsel::Vddbst
    }
}
#[doc = "Field `VSEL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type VselW<'a, REG> = crate::BitWriter<'a, REG, Vsel>;
impl<'a, REG> VselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(Vsel::Vdda)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddbst(self) -> &'a mut crate::W<REG> {
        self.variant(Vsel::Vddbst)
    }
}
#[doc = "Field `RESERVED9` reader - 23:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn shtvr0(&self) -> Shtvr0R {
        Shtvr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn shtvr1(&self) -> Shtvr1R {
        Shtvr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn shtva0(&self) -> Shtva0R {
        Shtva0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn shtva1(&self) -> Shtva1R {
        Shtva1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vr2va0(&self) -> Vr2va0R {
        Vr2va0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vr2va1(&self) -> Vr2va1R {
        Vr2va1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va2va0(&self) -> Va2va0R {
        Va2va0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va2va1(&self) -> Va2va1R {
        Va2va1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsel(&self) -> VselR {
        VselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:23 - 23:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn shtvr0(&mut self) -> Shtvr0W<AtestcfgSpec> {
        Shtvr0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn shtvr1(&mut self) -> Shtvr1W<AtestcfgSpec> {
        Shtvr1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn shtva0(&mut self) -> Shtva0W<AtestcfgSpec> {
        Shtva0W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn shtva1(&mut self) -> Shtva1W<AtestcfgSpec> {
        Shtva1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vr2va0(&mut self) -> Vr2va0W<AtestcfgSpec> {
        Vr2va0W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vr2va1(&mut self) -> Vr2va1W<AtestcfgSpec> {
        Vr2va1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn va2va0(&mut self) -> Va2va0W<AtestcfgSpec> {
        Va2va0W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn va2va1(&mut self) -> Va2va1W<AtestcfgSpec> {
        Va2va1W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VselW<AtestcfgSpec> {
        VselW::new(self, 8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<AtestcfgSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atestcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atestcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtestcfgSpec;
impl crate::RegisterSpec for AtestcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atestcfg::R`](R) reader structure"]
impl crate::Readable for AtestcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`atestcfg::W`](W) writer structure"]
impl crate::Writable for AtestcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATESTCFG to value 0x0f"]
impl crate::Resettable for AtestcfgSpec {
    const RESET_VALUE: u32 = 0x0f;
}
