#[doc = "Register `ANABYPASSVAL2` reader"]
pub type R = crate::R<Anabypassval2Spec>;
#[doc = "Register `ANABYPASSVAL2` writer"]
pub type W = crate::W<Anabypassval2Spec>;
#[doc = "Field `XOSC_HF_IBIASTHERM` reader - 13:0\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfIbiasthermR = crate::FieldReader<u16>;
#[doc = "Field `XOSC_HF_IBIASTHERM` writer - 13:0\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfIbiasthermW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&self) -> XoscHfIbiasthermR {
        XoscHfIbiasthermR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_ibiastherm(&mut self) -> XoscHfIbiasthermW<Anabypassval2Spec> {
        XoscHfIbiasthermW::new(self, 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<Anabypassval2Spec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anabypassval2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anabypassval2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Anabypassval2Spec;
impl crate::RegisterSpec for Anabypassval2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anabypassval2::R`](R) reader structure"]
impl crate::Readable for Anabypassval2Spec {}
#[doc = "`write(|w| ..)` method takes [`anabypassval2::W`](W) writer structure"]
impl crate::Writable for Anabypassval2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANABYPASSVAL2 to value 0"]
impl crate::Resettable for Anabypassval2Spec {
    const RESET_VALUE: u32 = 0;
}
