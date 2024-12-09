#[doc = "Register `ANABYPASS_VALUE2` reader"]
pub type R = crate::R<AnabypassValue2Spec>;
#[doc = "Register `ANABYPASS_VALUE2` writer"]
pub type W = crate::W<AnabypassValue2Spec>;
#[doc = "Field `XOSC_HF_IBIASTHERM` reader - 13:0\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfIbiasthermR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&self) -> XoscHfIbiasthermR {
        XoscHfIbiasthermR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anabypass_value2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anabypass_value2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnabypassValue2Spec;
impl crate::RegisterSpec for AnabypassValue2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anabypass_value2::R`](R) reader structure"]
impl crate::Readable for AnabypassValue2Spec {}
#[doc = "`write(|w| ..)` method takes [`anabypass_value2::W`](W) writer structure"]
impl crate::Writable for AnabypassValue2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANABYPASS_VALUE2 to value 0xffff_c3ff"]
impl crate::Resettable for AnabypassValue2Spec {
    const RESET_VALUE: u32 = 0xffff_c3ff;
}
