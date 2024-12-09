#[doc = "Register `STMPWCNTCAPT1` reader"]
pub type R = crate::R<Stmpwcntcapt1Spec>;
#[doc = "Register `STMPWCNTCAPT1` writer"]
pub type W = crate::W<Stmpwcntcapt1Spec>;
#[doc = "Field `CAPT_VALUE` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type CaptValueR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn capt_value(&self) -> CaptValueR {
        CaptValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwcntcapt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwcntcapt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stmpwcntcapt1Spec;
impl crate::RegisterSpec for Stmpwcntcapt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpwcntcapt1::R`](R) reader structure"]
impl crate::Readable for Stmpwcntcapt1Spec {}
#[doc = "`write(|w| ..)` method takes [`stmpwcntcapt1::W`](W) writer structure"]
impl crate::Writable for Stmpwcntcapt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPWCNTCAPT1 to value 0"]
impl crate::Resettable for Stmpwcntcapt1Spec {
    const RESET_VALUE: u32 = 0;
}
