#[doc = "Register `INT_CAUS` reader"]
pub type R = crate::R<IntCausSpec>;
#[doc = "Register `INT_CAUS` writer"]
pub type W = crate::W<IntCausSpec>;
#[doc = "Field `CAUSE_INTR` reader - 0:0\\]
Replica of RIS.WDTRIS"]
pub type CauseIntrR = crate::BitReader;
#[doc = "Field `CAUSE_RESET` reader - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
pub type CauseResetR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Replica of RIS.WDTRIS"]
    #[inline(always)]
    pub fn cause_intr(&self) -> CauseIntrR {
        CauseIntrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    pub fn cause_reset(&self) -> CauseResetR {
        CauseResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {}
#[doc = "Interrupt Cause Test Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_caus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_caus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntCausSpec;
impl crate::RegisterSpec for IntCausSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_caus::R`](R) reader structure"]
impl crate::Readable for IntCausSpec {}
#[doc = "`write(|w| ..)` method takes [`int_caus::W`](W) writer structure"]
impl crate::Writable for IntCausSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CAUS to value 0"]
impl crate::Resettable for IntCausSpec {
    const RESET_VALUE: u32 = 0;
}
