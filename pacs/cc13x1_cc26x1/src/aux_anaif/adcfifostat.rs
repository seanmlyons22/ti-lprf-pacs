#[doc = "Register `ADCFIFOSTAT` reader"]
pub type R = crate::R<AdcfifostatSpec>;
#[doc = "Register `ADCFIFOSTAT` writer"]
pub type W = crate::W<AdcfifostatSpec>;
#[doc = "Field `EMPTY` reader - 0:0\\]
FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
pub type EmptyR = crate::BitReader;
#[doc = "Field `ALMOST_FULL` reader - 1:1\\]
FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
pub type AlmostFullR = crate::BitReader;
#[doc = "Field `FULL` reader - 2:2\\]
FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
pub type FullR = crate::BitReader;
#[doc = "Field `UNDERFLOW` reader - 3:3\\]
FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` reader - 4:4\\]
FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
pub type OverflowR = crate::BitReader;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
    #[inline(always)]
    pub fn almost_full(&self) -> AlmostFullR {
        AlmostFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {}
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcfifostat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcfifostat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcfifostatSpec;
impl crate::RegisterSpec for AdcfifostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcfifostat::R`](R) reader structure"]
impl crate::Readable for AdcfifostatSpec {}
#[doc = "`write(|w| ..)` method takes [`adcfifostat::W`](W) writer structure"]
impl crate::Writable for AdcfifostatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCFIFOSTAT to value 0x01"]
impl crate::Resettable for AdcfifostatSpec {
    const RESET_VALUE: u32 = 0x01;
}
