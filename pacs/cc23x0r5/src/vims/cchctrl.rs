#[doc = "Register `CCHCTRL` reader"]
pub type R = crate::R<CchctrlSpec>;
#[doc = "Register `CCHCTRL` writer"]
pub type W = crate::W<CchctrlSpec>;
#[doc = "0:0\\]
This bit is used to enable the cache.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cchen {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Cchen> for bool {
    #[inline(always)]
    fn from(variant: Cchen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCHEN` reader - 0:0\\]
This bit is used to enable the cache."]
pub type CchenR = crate::BitReader<Cchen>;
impl CchenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cchen {
        match self.bits {
            true => Cchen::Enable,
            false => Cchen::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cchen::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cchen::Disable
    }
}
#[doc = "Field `CCHEN` writer - 0:0\\]
This bit is used to enable the cache."]
pub type CchenW<'a, REG> = crate::BitWriter<'a, REG, Cchen>;
impl<'a, REG> CchenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cchen::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cchen::Disable)
    }
}
#[doc = "1:1\\]
This bit is used to enable the prefetch unit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cchpfen {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Cchpfen> for bool {
    #[inline(always)]
    fn from(variant: Cchpfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCHPFEN` reader - 1:1\\]
This bit is used to enable the prefetch unit."]
pub type CchpfenR = crate::BitReader<Cchpfen>;
impl CchpfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cchpfen {
        match self.bits {
            true => Cchpfen::Enable,
            false => Cchpfen::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cchpfen::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cchpfen::Disable
    }
}
#[doc = "Field `CCHPFEN` writer - 1:1\\]
This bit is used to enable the prefetch unit."]
pub type CchpfenW<'a, REG> = crate::BitWriter<'a, REG, Cchpfen>;
impl<'a, REG> CchpfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cchpfen::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cchpfen::Disable)
    }
}
#[doc = "2:2\\]
This bit is used to enable the micropredictor unit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cchmpen {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Cchmpen> for bool {
    #[inline(always)]
    fn from(variant: Cchmpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCHMPEN` reader - 2:2\\]
This bit is used to enable the micropredictor unit."]
pub type CchmpenR = crate::BitReader<Cchmpen>;
impl CchmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cchmpen {
        match self.bits {
            true => Cchmpen::Enable,
            false => Cchmpen::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cchmpen::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cchmpen::Disable
    }
}
#[doc = "Field `CCHMPEN` writer - 2:2\\]
This bit is used to enable the micropredictor unit."]
pub type CchmpenW<'a, REG> = crate::BitWriter<'a, REG, Cchmpen>;
impl<'a, REG> CchmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cchmpen::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cchmpen::Disable)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to enable the cache."]
    #[inline(always)]
    pub fn cchen(&self) -> CchenR {
        CchenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is used to enable the prefetch unit."]
    #[inline(always)]
    pub fn cchpfen(&self) -> CchpfenR {
        CchpfenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is used to enable the micropredictor unit."]
    #[inline(always)]
    pub fn cchmpen(&self) -> CchmpenR {
        CchmpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to enable the cache."]
    #[inline(always)]
    #[must_use]
    pub fn cchen(&mut self) -> CchenW<CchctrlSpec> {
        CchenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is used to enable the prefetch unit."]
    #[inline(always)]
    #[must_use]
    pub fn cchpfen(&mut self) -> CchpfenW<CchctrlSpec> {
        CchpfenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is used to enable the micropredictor unit."]
    #[inline(always)]
    #[must_use]
    pub fn cchmpen(&mut self) -> CchmpenW<CchctrlSpec> {
        CchmpenW::new(self, 2)
    }
}
#[doc = "This register is used for enabling cache, prefetch $amp;amp; micropredictor units. This register is retained\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cchctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cchctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CchctrlSpec;
impl crate::RegisterSpec for CchctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cchctrl::R`](R) reader structure"]
impl crate::Readable for CchctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cchctrl::W`](W) writer structure"]
impl crate::Writable for CchctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCHCTRL to value 0x07"]
impl crate::Resettable for CchctrlSpec {
    const RESET_VALUE: u32 = 0x07;
}
