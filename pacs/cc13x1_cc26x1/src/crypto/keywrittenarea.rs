#[doc = "Register `KEYWRITTENAREA` reader"]
pub type R = crate::R<KeywrittenareaSpec>;
#[doc = "Register `KEYWRITTENAREA` writer"]
pub type W = crate::W<KeywrittenareaSpec>;
#[doc = "0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten0 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten0> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN0` reader - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten0R = crate::BitReader<RamAreaWritten0>;
impl RamAreaWritten0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten0 {
        match self.bits {
            true => RamAreaWritten0::Written,
            false => RamAreaWritten0::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten0::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten0::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN0` writer - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten0W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten0>;
impl<'a, REG> RamAreaWritten0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten0::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten0::NotWritten)
    }
}
#[doc = "1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten1 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten1> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN1` reader - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten1R = crate::BitReader<RamAreaWritten1>;
impl RamAreaWritten1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten1 {
        match self.bits {
            true => RamAreaWritten1::Written,
            false => RamAreaWritten1::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten1::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten1::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN1` writer - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten1W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten1>;
impl<'a, REG> RamAreaWritten1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten1::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten1::NotWritten)
    }
}
#[doc = "2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten2 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten2> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN2` reader - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten2R = crate::BitReader<RamAreaWritten2>;
impl RamAreaWritten2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten2 {
        match self.bits {
            true => RamAreaWritten2::Written,
            false => RamAreaWritten2::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten2::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten2::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN2` writer - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten2W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten2>;
impl<'a, REG> RamAreaWritten2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten2::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten2::NotWritten)
    }
}
#[doc = "3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten3 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten3> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN3` reader - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten3R = crate::BitReader<RamAreaWritten3>;
impl RamAreaWritten3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten3 {
        match self.bits {
            true => RamAreaWritten3::Written,
            false => RamAreaWritten3::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten3::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten3::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN3` writer - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten3W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten3>;
impl<'a, REG> RamAreaWritten3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten3::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten3::NotWritten)
    }
}
#[doc = "4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten4 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten4> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN4` reader - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten4R = crate::BitReader<RamAreaWritten4>;
impl RamAreaWritten4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten4 {
        match self.bits {
            true => RamAreaWritten4::Written,
            false => RamAreaWritten4::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten4::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten4::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN4` writer - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten4W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten4>;
impl<'a, REG> RamAreaWritten4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten4::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten4::NotWritten)
    }
}
#[doc = "5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten5 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten5> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN5` reader - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten5R = crate::BitReader<RamAreaWritten5>;
impl RamAreaWritten5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten5 {
        match self.bits {
            true => RamAreaWritten5::Written,
            false => RamAreaWritten5::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten5::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten5::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN5` writer - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten5W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten5>;
impl<'a, REG> RamAreaWritten5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten5::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten5::NotWritten)
    }
}
#[doc = "6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten6 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten6> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN6` reader - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten6R = crate::BitReader<RamAreaWritten6>;
impl RamAreaWritten6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten6 {
        match self.bits {
            true => RamAreaWritten6::Written,
            false => RamAreaWritten6::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten6::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten6::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN6` writer - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten6W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten6>;
impl<'a, REG> RamAreaWritten6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten6::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten6::NotWritten)
    }
}
#[doc = "7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamAreaWritten7 {
    #[doc = "1: This RAM area is written with valid key information"]
    Written = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NotWritten = 0,
}
impl From<RamAreaWritten7> for bool {
    #[inline(always)]
    fn from(variant: RamAreaWritten7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN7` reader - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten7R = crate::BitReader<RamAreaWritten7>;
impl RamAreaWritten7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamAreaWritten7 {
        match self.bits {
            true => RamAreaWritten7::Written,
            false => RamAreaWritten7::NotWritten,
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        *self == RamAreaWritten7::Written
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        *self == RamAreaWritten7::NotWritten
    }
}
#[doc = "Field `RAM_AREA_WRITTEN7` writer - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RamAreaWritten7W<'a, REG> = crate::BitWriter<'a, REG, RamAreaWritten7>;
impl<'a, REG> RamAreaWritten7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten7::Written)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut crate::W<REG> {
        self.variant(RamAreaWritten7::NotWritten)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&self) -> RamAreaWritten0R {
        RamAreaWritten0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&self) -> RamAreaWritten1R {
        RamAreaWritten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&self) -> RamAreaWritten2R {
        RamAreaWritten2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&self) -> RamAreaWritten3R {
        RamAreaWritten3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&self) -> RamAreaWritten4R {
        RamAreaWritten4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&self) -> RamAreaWritten5R {
        RamAreaWritten5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&self) -> RamAreaWritten6R {
        RamAreaWritten6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&self) -> RamAreaWritten7R {
        RamAreaWritten7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written0(&mut self) -> RamAreaWritten0W<KeywrittenareaSpec> {
        RamAreaWritten0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written1(&mut self) -> RamAreaWritten1W<KeywrittenareaSpec> {
        RamAreaWritten1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written2(&mut self) -> RamAreaWritten2W<KeywrittenareaSpec> {
        RamAreaWritten2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written3(&mut self) -> RamAreaWritten3W<KeywrittenareaSpec> {
        RamAreaWritten3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written4(&mut self) -> RamAreaWritten4W<KeywrittenareaSpec> {
        RamAreaWritten4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written5(&mut self) -> RamAreaWritten5W<KeywrittenareaSpec> {
        RamAreaWritten5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written6(&mut self) -> RamAreaWritten6W<KeywrittenareaSpec> {
        RamAreaWritten6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written7(&mut self) -> RamAreaWritten7W<KeywrittenareaSpec> {
        RamAreaWritten7W::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<KeywrittenareaSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keywrittenarea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywrittenarea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeywrittenareaSpec;
impl crate::RegisterSpec for KeywrittenareaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keywrittenarea::R`](R) reader structure"]
impl crate::Readable for KeywrittenareaSpec {}
#[doc = "`write(|w| ..)` method takes [`keywrittenarea::W`](W) writer structure"]
impl crate::Writable for KeywrittenareaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYWRITTENAREA to value 0"]
impl crate::Resettable for KeywrittenareaSpec {
    const RESET_VALUE: u32 = 0;
}
