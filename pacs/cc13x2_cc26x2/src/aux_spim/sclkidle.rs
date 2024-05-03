#[doc = "Register `SCLKIDLE` reader"]
pub type R = crate::R<SclkidleSpec>;
#[doc = "Register `SCLKIDLE` writer"]
pub type W = crate::W<SclkidleSpec>;
#[doc = "Field `STAT` reader - 0:0\\]
Wait for SCLK idle. Read operation stalls until SCLK is idle with no remaining clock edges. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
pub type StatR = crate::BitReader;
#[doc = "Field `STAT` writer - 0:0\\]
Wait for SCLK idle. Read operation stalls until SCLK is idle with no remaining clock edges. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Wait for SCLK idle. Read operation stalls until SCLK is idle with no remaining clock edges. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 1) != 0)
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
Wait for SCLK idle. Read operation stalls until SCLK is idle with no remaining clock edges. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<SclkidleSpec> {
        StatW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SclkidleSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sclkidle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sclkidle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclkidleSpec;
impl crate::RegisterSpec for SclkidleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sclkidle::R`](R) reader structure"]
impl crate::Readable for SclkidleSpec {}
#[doc = "`write(|w| ..)` method takes [`sclkidle::W`](W) writer structure"]
impl crate::Writable for SclkidleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCLKIDLE to value 0x01"]
impl crate::Resettable for SclkidleSpec {
    const RESET_VALUE: u32 = 0x01;
}
