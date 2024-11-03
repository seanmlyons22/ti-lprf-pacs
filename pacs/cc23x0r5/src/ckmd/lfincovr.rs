#[doc = "Register `LFINCOVR` reader"]
pub type R = crate::R<LfincovrSpec>;
#[doc = "Register `LFINCOVR` writer"]
pub type W = crate::W<LfincovrSpec>;
#[doc = "Field `LFINC` reader - 21:0\\]
LF increment value This value is used when OVERRIDE is set to 1. Otherwise the value is calculated automatically."]
pub type LfincR = crate::FieldReader<u32>;
#[doc = "Field `LFINC` writer - 21:0\\]
LF increment value This value is used when OVERRIDE is set to 1. Otherwise the value is calculated automatically."]
pub type LfincW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `RESERVED22` reader - 30:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED22` writer - 30:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `OVERRIDE` reader - 31:31\\]
Override LF increment Use the value provided in LFINC instead of the value calculated by Hardware."]
pub type OverrideR = crate::BitReader;
#[doc = "Field `OVERRIDE` writer - 31:31\\]
Override LF increment Use the value provided in LFINC instead of the value calculated by Hardware."]
pub type OverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - 21:0\\]
LF increment value This value is used when OVERRIDE is set to 1. Otherwise the value is calculated automatically."]
    #[inline(always)]
    pub fn lfinc(&self) -> LfincR {
        LfincR::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:30 - 30:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Override LF increment Use the value provided in LFINC instead of the value calculated by Hardware."]
    #[inline(always)]
    pub fn override_(&self) -> OverrideR {
        OverrideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - 21:0\\]
LF increment value This value is used when OVERRIDE is set to 1. Otherwise the value is calculated automatically."]
    #[inline(always)]
    #[must_use]
    pub fn lfinc(&mut self) -> LfincW<LfincovrSpec> {
        LfincW::new(self, 0)
    }
    #[doc = "Bits 22:30 - 30:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> Reserved22W<LfincovrSpec> {
        Reserved22W::new(self, 22)
    }
    #[doc = "Bit 31 - 31:31\\]
Override LF increment Use the value provided in LFINC instead of the value calculated by Hardware."]
    #[inline(always)]
    #[must_use]
    pub fn override_(&mut self) -> OverrideW<LfincovrSpec> {
        OverrideW::new(self, 31)
    }
}
#[doc = "Low frequency time increment override control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfincovr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfincovr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfincovrSpec;
impl crate::RegisterSpec for LfincovrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfincovr::R`](R) reader structure"]
impl crate::Readable for LfincovrSpec {}
#[doc = "`write(|w| ..)` method takes [`lfincovr::W`](W) writer structure"]
impl crate::Writable for LfincovrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFINCOVR to value 0"]
impl crate::Resettable for LfincovrSpec {
    const RESET_VALUE: u32 = 0;
}
