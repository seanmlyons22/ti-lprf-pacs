#[doc = "Register `LPMBIASCTL` reader"]
pub type R = crate::R<LpmbiasctlSpec>;
#[doc = "Register `LPMBIASCTL` writer"]
pub type W = crate::W<LpmbiasctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Module enable. 0: Disable low power mode bias module. 1: Enable low power mode bias module. Set EN to 1 15 us before you enable the DAC or Comparator A."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Module enable. 0: Disable low power mode bias module. 1: Enable low power mode bias module. Set EN to 1 15 us before you enable the DAC or Comparator A."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Module enable. 0: Disable low power mode bias module. 1: Enable low power mode bias module. Set EN to 1 15 us before you enable the DAC or Comparator A."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Module enable. 0: Disable low power mode bias module. 1: Enable low power mode bias module. Set EN to 1 15 us before you enable the DAC or Comparator A."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<LpmbiasctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<LpmbiasctlSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmbiasctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmbiasctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmbiasctlSpec;
impl crate::RegisterSpec for LpmbiasctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmbiasctl::R`](R) reader structure"]
impl crate::Readable for LpmbiasctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmbiasctl::W`](W) writer structure"]
impl crate::Writable for LpmbiasctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPMBIASCTL to value 0"]
impl crate::Resettable for LpmbiasctlSpec {
    const RESET_VALUE: u32 = 0;
}
