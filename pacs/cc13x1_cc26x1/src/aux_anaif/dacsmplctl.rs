#[doc = "Register `DACSMPLCTL` reader"]
pub type R = crate::R<DacsmplctlSpec>;
#[doc = "Register `DACSMPLCTL` writer"]
pub type W = crate::W<DacsmplctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DacsmplctlSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacsmplctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacsmplctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacsmplctlSpec;
impl crate::RegisterSpec for DacsmplctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacsmplctl::R`](R) reader structure"]
impl crate::Readable for DacsmplctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dacsmplctl::W`](W) writer structure"]
impl crate::Writable for DacsmplctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACSMPLCTL to value 0"]
impl crate::Resettable for DacsmplctlSpec {
    const RESET_VALUE: u32 = 0;
}
