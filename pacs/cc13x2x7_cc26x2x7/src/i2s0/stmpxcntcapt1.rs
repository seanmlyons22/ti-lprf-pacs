#[doc = "Register `STMPXCNTCAPT1` reader"]
pub type R = crate::R<Stmpxcntcapt1Spec>;
#[doc = "Register `STMPXCNTCAPT1` writer"]
pub type W = crate::W<Stmpxcntcapt1Spec>;
#[doc = "Field `CAPT_VALUE` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type CaptValueR = crate::FieldReader<u16>;
#[doc = "Field `CAPT_VALUE` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type CaptValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn capt_value(&mut self) -> CaptValueW<Stmpxcntcapt1Spec> {
        CaptValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Stmpxcntcapt1Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxcntcapt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxcntcapt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stmpxcntcapt1Spec;
impl crate::RegisterSpec for Stmpxcntcapt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpxcntcapt1::R`](R) reader structure"]
impl crate::Readable for Stmpxcntcapt1Spec {}
#[doc = "`write(|w| ..)` method takes [`stmpxcntcapt1::W`](W) writer structure"]
impl crate::Writable for Stmpxcntcapt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPXCNTCAPT1 to value 0"]
impl crate::Resettable for Stmpxcntcapt1Spec {
    const RESET_VALUE: u32 = 0;
}
