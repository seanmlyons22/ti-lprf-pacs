#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CpsrSpec>;
#[doc = "Register `CPSR` writer"]
pub type W = crate::W<CpsrSpec>;
#[doc = "Field `CPSDVSR` reader - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
pub type CpsdvsrR = crate::FieldReader;
#[doc = "Field `CPSDVSR` writer - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
pub type CpsdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    #[must_use]
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<CpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
#[doc = "Clock Prescale\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsrSpec;
impl crate::RegisterSpec for CpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsr::W`](W) writer structure"]
impl crate::Writable for CpsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CpsrSpec {
    const RESET_VALUE: u32 = 0;
}
