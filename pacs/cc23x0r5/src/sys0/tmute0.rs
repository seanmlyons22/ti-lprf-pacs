#[doc = "Register `TMUTE0` reader"]
pub type R = crate::R<Tmute0Spec>;
#[doc = "Register `TMUTE0` writer"]
pub type W = crate::W<Tmute0Spec>;
#[doc = "Field `CDACL` reader - 31:0\\]
SOCADC: Lower 32 bits of CDAC trim. These bits are used in DTC."]
pub type CdaclR = crate::FieldReader<u32>;
#[doc = "Field `CDACL` writer - 31:0\\]
SOCADC: Lower 32 bits of CDAC trim. These bits are used in DTC."]
pub type CdaclW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SOCADC: Lower 32 bits of CDAC trim. These bits are used in DTC."]
    #[inline(always)]
    pub fn cdacl(&self) -> CdaclR {
        CdaclR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SOCADC: Lower 32 bits of CDAC trim. These bits are used in DTC."]
    #[inline(always)]
    #[must_use]
    pub fn cdacl(&mut self) -> CdaclW<Tmute0Spec> {
        CdaclW::new(self, 0)
    }
}
#[doc = "TMUTE0 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmute0Spec;
impl crate::RegisterSpec for Tmute0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmute0::R`](R) reader structure"]
impl crate::Readable for Tmute0Spec {}
#[doc = "`write(|w| ..)` method takes [`tmute0::W`](W) writer structure"]
impl crate::Writable for Tmute0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMUTE0 to value 0"]
impl crate::Resettable for Tmute0Spec {
    const RESET_VALUE: u32 = 0;
}
