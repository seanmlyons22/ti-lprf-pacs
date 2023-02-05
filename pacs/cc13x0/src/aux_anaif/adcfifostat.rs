#[doc = "Register `ADCFIFOSTAT` reader"]
pub struct R(crate::R<ADCFIFOSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCFIFOSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCFIFOSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCFIFOSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCFIFOSTAT` writer"]
pub struct W(crate::W<ADCFIFOSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCFIFOSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADCFIFOSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCFIFOSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMPTY` reader - 0:0\\]
FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY` writer - 0:0\\]
FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
pub type EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCFIFOSTAT_SPEC, bool, O>;
#[doc = "Field `ALMOST_FULL` reader - 1:1\\]
FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
pub type ALMOST_FULL_R = crate::BitReader<bool>;
#[doc = "Field `ALMOST_FULL` writer - 1:1\\]
FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
pub type ALMOST_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCFIFOSTAT_SPEC, bool, O>;
#[doc = "Field `FULL` reader - 2:2\\]
FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `FULL` writer - 2:2\\]
FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCFIFOSTAT_SPEC, bool, O>;
#[doc = "Field `UNDERFLOW` reader - 3:3\\]
FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
pub type UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFLOW` writer - 3:3\\]
FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
pub type UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCFIFOSTAT_SPEC, bool, O>;
#[doc = "Field `OVERFLOW` reader - 4:4\\]
FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
pub type OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW` writer - 4:4\\]
FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
pub type OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCFIFOSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCFIFOSTAT_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
    #[inline(always)]
    pub fn almost_full(&self) -> ALMOST_FULL_R {
        ALMOST_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<0> {
        EMPTY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
    #[inline(always)]
    #[must_use]
    pub fn almost_full(&mut self) -> ALMOST_FULL_W<1> {
        ALMOST_FULL_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<2> {
        FULL_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<3> {
        UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<4> {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcfifostat](index.html) module"]
pub struct ADCFIFOSTAT_SPEC;
impl crate::RegisterSpec for ADCFIFOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcfifostat::R](R) reader structure"]
impl crate::Readable for ADCFIFOSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcfifostat::W](W) writer structure"]
impl crate::Writable for ADCFIFOSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCFIFOSTAT to value 0x01"]
impl crate::Resettable for ADCFIFOSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
