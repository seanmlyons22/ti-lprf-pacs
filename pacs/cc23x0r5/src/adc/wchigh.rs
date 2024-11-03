#[doc = "Register `WCHIGH` reader"]
pub type R = crate::R<WchighSpec>;
#[doc = "Register `WCHIGH` writer"]
pub type W = crate::W<WchighSpec>;
#[doc = "Field `DATA` reader - 15:0\\]
If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 15:0\\]
If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<WchighSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<WchighSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Window Comparator High Threshold Register. The data format that is used to write and read WCHIGH depends on the value of DF bit in CTL2 register. CTL0.ENC must be 0 to write to this register. Note: Change in ADC data format or resolution does not reset WCHIGH.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wchigh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wchigh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WchighSpec;
impl crate::RegisterSpec for WchighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wchigh::R`](R) reader structure"]
impl crate::Readable for WchighSpec {}
#[doc = "`write(|w| ..)` method takes [`wchigh::W`](W) writer structure"]
impl crate::Writable for WchighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WCHIGH to value 0"]
impl crate::Resettable for WchighSpec {
    const RESET_VALUE: u32 = 0;
}
