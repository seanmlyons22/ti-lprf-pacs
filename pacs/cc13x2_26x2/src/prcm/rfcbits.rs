#[doc = "Register `RFCBITS` reader"]
pub type R = crate::R<RfcbitsSpec>;
#[doc = "Register `RFCBITS` writer"]
pub type W = crate::W<RfcbitsSpec>;
#[doc = "Field `READ` reader - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
pub type ReadR = crate::FieldReader<u32>;
#[doc = "Field `READ` writer - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
pub type ReadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<RfcbitsSpec> {
        ReadW::new(self, 0)
    }
}
#[doc = "Control To RFC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcbits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcbits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcbitsSpec;
impl crate::RegisterSpec for RfcbitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcbits::R`](R) reader structure"]
impl crate::Readable for RfcbitsSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcbits::W`](W) writer structure"]
impl crate::Writable for RfcbitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCBITS to value 0"]
impl crate::Resettable for RfcbitsSpec {
    const RESET_VALUE: u32 = 0;
}
