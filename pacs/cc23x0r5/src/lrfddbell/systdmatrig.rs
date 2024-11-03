#[doc = "Register `SYSTDMATRIG` reader"]
pub type R = crate::R<SystdmatrigSpec>;
#[doc = "Register `SYSTDMATRIG` writer"]
pub type W = crate::W<SystdmatrigSpec>;
#[doc = "0:0\\]
Trigger a capture event on systimer event 0 from the radio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syst0 {
    #[doc = "1: Capture event triggered"]
    Trig = 1,
    #[doc = "0: Not capture event triggered"]
    Notrig = 0,
}
impl From<Syst0> for bool {
    #[inline(always)]
    fn from(variant: Syst0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYST0` reader - 0:0\\]
Trigger a capture event on systimer event 0 from the radio"]
pub type Syst0R = crate::BitReader<Syst0>;
impl Syst0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syst0 {
        match self.bits {
            true => Syst0::Trig,
            false => Syst0::Notrig,
        }
    }
    #[doc = "Capture event triggered"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Syst0::Trig
    }
    #[doc = "Not capture event triggered"]
    #[inline(always)]
    pub fn is_notrig(&self) -> bool {
        *self == Syst0::Notrig
    }
}
#[doc = "Field `SYST0` writer - 0:0\\]
Trigger a capture event on systimer event 0 from the radio"]
pub type Syst0W<'a, REG> = crate::BitWriter<'a, REG, Syst0>;
impl<'a, REG> Syst0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture event triggered"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Syst0::Trig)
    }
    #[doc = "Not capture event triggered"]
    #[inline(always)]
    pub fn notrig(self) -> &'a mut crate::W<REG> {
        self.variant(Syst0::Notrig)
    }
}
#[doc = "1:1\\]
Trigger a capture event on systimer event 0 from the radio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syst1 {
    #[doc = "1: Capture event triggered"]
    Trig = 1,
    #[doc = "0: Not capture event triggered"]
    Notrig = 0,
}
impl From<Syst1> for bool {
    #[inline(always)]
    fn from(variant: Syst1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYST1` reader - 1:1\\]
Trigger a capture event on systimer event 0 from the radio"]
pub type Syst1R = crate::BitReader<Syst1>;
impl Syst1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syst1 {
        match self.bits {
            true => Syst1::Trig,
            false => Syst1::Notrig,
        }
    }
    #[doc = "Capture event triggered"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Syst1::Trig
    }
    #[doc = "Not capture event triggered"]
    #[inline(always)]
    pub fn is_notrig(&self) -> bool {
        *self == Syst1::Notrig
    }
}
#[doc = "Field `SYST1` writer - 1:1\\]
Trigger a capture event on systimer event 0 from the radio"]
pub type Syst1W<'a, REG> = crate::BitWriter<'a, REG, Syst1>;
impl<'a, REG> Syst1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture event triggered"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Syst1::Trig)
    }
    #[doc = "Not capture event triggered"]
    #[inline(always)]
    pub fn notrig(self) -> &'a mut crate::W<REG> {
        self.variant(Syst1::Notrig)
    }
}
#[doc = "2:2\\]
Trigger a capture event on systimer event 0 from the radio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syst2 {
    #[doc = "1: Capture event triggered"]
    Trig = 1,
    #[doc = "0: Not capture event triggered"]
    Notrig = 0,
}
impl From<Syst2> for bool {
    #[inline(always)]
    fn from(variant: Syst2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYST2` reader - 2:2\\]
Trigger a capture event on systimer event 0 from the radio"]
pub type Syst2R = crate::BitReader<Syst2>;
impl Syst2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syst2 {
        match self.bits {
            true => Syst2::Trig,
            false => Syst2::Notrig,
        }
    }
    #[doc = "Capture event triggered"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Syst2::Trig
    }
    #[doc = "Not capture event triggered"]
    #[inline(always)]
    pub fn is_notrig(&self) -> bool {
        *self == Syst2::Notrig
    }
}
#[doc = "Field `SYST2` writer - 2:2\\]
Trigger a capture event on systimer event 0 from the radio"]
pub type Syst2W<'a, REG> = crate::BitWriter<'a, REG, Syst2>;
impl<'a, REG> Syst2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture event triggered"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Syst2::Trig)
    }
    #[doc = "Not capture event triggered"]
    #[inline(always)]
    pub fn notrig(self) -> &'a mut crate::W<REG> {
        self.variant(Syst2::Notrig)
    }
}
#[doc = "3:3\\]
Trigger a DMA request from the Radio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "1: DMA request manually triggered"]
    Trig = 1,
    #[doc = "0: DMA not manually triggered"]
    Notrig = 0,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - 3:3\\]
Trigger a DMA request from the Radio"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            true => Dma::Trig,
            false => Dma::Notrig,
        }
    }
    #[doc = "DMA request manually triggered"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Dma::Trig
    }
    #[doc = "DMA not manually triggered"]
    #[inline(always)]
    pub fn is_notrig(&self) -> bool {
        *self == Dma::Notrig
    }
}
#[doc = "Field `DMA` writer - 3:3\\]
Trigger a DMA request from the Radio"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request manually triggered"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Trig)
    }
    #[doc = "DMA not manually triggered"]
    #[inline(always)]
    pub fn notrig(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Notrig)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Trigger a capture event on systimer event 0 from the radio"]
    #[inline(always)]
    pub fn syst0(&self) -> Syst0R {
        Syst0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Trigger a capture event on systimer event 0 from the radio"]
    #[inline(always)]
    pub fn syst1(&self) -> Syst1R {
        Syst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Trigger a capture event on systimer event 0 from the radio"]
    #[inline(always)]
    pub fn syst2(&self) -> Syst2R {
        Syst2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Trigger a DMA request from the Radio"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Trigger a capture event on systimer event 0 from the radio"]
    #[inline(always)]
    #[must_use]
    pub fn syst0(&mut self) -> Syst0W<SystdmatrigSpec> {
        Syst0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Trigger a capture event on systimer event 0 from the radio"]
    #[inline(always)]
    #[must_use]
    pub fn syst1(&mut self) -> Syst1W<SystdmatrigSpec> {
        Syst1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Trigger a capture event on systimer event 0 from the radio"]
    #[inline(always)]
    #[must_use]
    pub fn syst2(&mut self) -> Syst2W<SystdmatrigSpec> {
        Syst2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Trigger a DMA request from the Radio"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<SystdmatrigSpec> {
        DmaW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SystdmatrigSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Manual triggering of systimer capture event or DMA trigger This comes on top of any HW driven sources configured in SYSTIMOEV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systdmatrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systdmatrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystdmatrigSpec;
impl crate::RegisterSpec for SystdmatrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systdmatrig::R`](R) reader structure"]
impl crate::Readable for SystdmatrigSpec {}
#[doc = "`write(|w| ..)` method takes [`systdmatrig::W`](W) writer structure"]
impl crate::Writable for SystdmatrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTDMATRIG to value 0"]
impl crate::Resettable for SystdmatrigSpec {
    const RESET_VALUE: u32 = 0;
}
