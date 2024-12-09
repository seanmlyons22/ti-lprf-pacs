#[doc = "Register `HWOPT` reader"]
pub type R = crate::R<HwoptSpec>;
#[doc = "Register `HWOPT` writer"]
pub type W = crate::W<HwoptSpec>;
#[doc = "Field `RESERVED0` reader - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `NR_OF_FROS` reader - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
pub type NrOfFrosR = crate::FieldReader;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    pub fn nr_of_fros(&self) -> NrOfFrosR {
        NrOfFrosR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "TRNG Engine Options Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwopt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwopt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwoptSpec;
impl crate::RegisterSpec for HwoptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwopt::R`](R) reader structure"]
impl crate::Readable for HwoptSpec {}
#[doc = "`write(|w| ..)` method takes [`hwopt::W`](W) writer structure"]
impl crate::Writable for HwoptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWOPT to value 0x0600"]
impl crate::Resettable for HwoptSpec {
    const RESET_VALUE: u32 = 0x0600;
}
