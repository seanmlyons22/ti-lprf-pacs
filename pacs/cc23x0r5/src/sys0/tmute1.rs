#[doc = "Register `TMUTE1` reader"]
pub type R = crate::R<Tmute1Spec>;
#[doc = "Register `TMUTE1` writer"]
pub type W = crate::W<Tmute1Spec>;
#[doc = "Field `CDACM` reader - 31:0\\]
SOCADC: Middle 32 bits of CDAC trim. These bits are used in DTC."]
pub type CdacmR = crate::FieldReader<u32>;
#[doc = "Field `CDACM` writer - 31:0\\]
SOCADC: Middle 32 bits of CDAC trim. These bits are used in DTC."]
pub type CdacmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SOCADC: Middle 32 bits of CDAC trim. These bits are used in DTC."]
    #[inline(always)]
    pub fn cdacm(&self) -> CdacmR {
        CdacmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SOCADC: Middle 32 bits of CDAC trim. These bits are used in DTC."]
    #[inline(always)]
    #[must_use]
    pub fn cdacm(&mut self) -> CdacmW<Tmute1Spec> {
        CdacmW::new(self, 0)
    }
}
#[doc = "TMUTE1 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmute1Spec;
impl crate::RegisterSpec for Tmute1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmute1::R`](R) reader structure"]
impl crate::Readable for Tmute1Spec {}
#[doc = "`write(|w| ..)` method takes [`tmute1::W`](W) writer structure"]
impl crate::Writable for Tmute1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMUTE1 to value 0"]
impl crate::Resettable for Tmute1Spec {
    const RESET_VALUE: u32 = 0;
}
