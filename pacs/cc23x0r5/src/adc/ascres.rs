#[doc = "Register `ASCRES` reader"]
pub type R = crate::R<AscresSpec>;
#[doc = "Register `ASCRES` writer"]
pub type W = crate::W<AscresSpec>;
#[doc = "Field `DATA` reader - 15:0\\]
Result of ADC ad-hoc single conversion. If DF = 0, unsigned binary: The conversion result is right aligned. In 10 and 8 bit modes, the unused MSB bits are forced to 0. If DF = 1, 2s-complement format: The conversion result is left aligned. In 10 and 8 bit modes, the unused LSB bits are forced to 0. The data is stored in the right-justified format and is converted to the left-justified 2s-complement format during read back."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Result of ADC ad-hoc single conversion. If DF = 0, unsigned binary: The conversion result is right aligned. In 10 and 8 bit modes, the unused MSB bits are forced to 0. If DF = 1, 2s-complement format: The conversion result is left aligned. In 10 and 8 bit modes, the unused LSB bits are forced to 0. The data is stored in the right-justified format and is converted to the left-justified 2s-complement format during read back."]
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
impl W {}
#[doc = "ASC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ascres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ascres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AscresSpec;
impl crate::RegisterSpec for AscresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ascres::R`](R) reader structure"]
impl crate::Readable for AscresSpec {}
#[doc = "`write(|w| ..)` method takes [`ascres::W`](W) writer structure"]
impl crate::Writable for AscresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASCRES to value 0"]
impl crate::Resettable for AscresSpec {
    const RESET_VALUE: u32 = 0;
}
