#[doc = "Register `FIFODATA` reader"]
pub type R = crate::R<FifodataSpec>;
#[doc = "Register `FIFODATA` writer"]
pub type W = crate::W<FifodataSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
Read from this data field returns the ADC sample from FIFO."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Read from this data field returns the ADC sample from FIFO."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Read from this data field returns the ADC sample from FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Read from this data field returns the ADC sample from FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<FifodataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "FIFO data register. This is a virtual register used to do read from FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifodata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifodata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifodataSpec;
impl crate::RegisterSpec for FifodataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodata::R`](R) reader structure"]
impl crate::Readable for FifodataSpec {}
#[doc = "`write(|w| ..)` method takes [`fifodata::W`](W) writer structure"]
impl crate::Writable for FifodataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFODATA to value 0"]
impl crate::Resettable for FifodataSpec {
    const RESET_VALUE: u32 = 0;
}
