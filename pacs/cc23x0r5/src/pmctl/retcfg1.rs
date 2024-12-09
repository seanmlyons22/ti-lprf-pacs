#[doc = "Register `RETCFG1` reader"]
pub type R = crate::R<Retcfg1Spec>;
#[doc = "Register `RETCFG1` writer"]
pub type W = crate::W<Retcfg1Spec>;
#[doc = "Field `VAL` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ValR = crate::BitReader;
#[doc = "Field `VAL` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Retcfg1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Retcfg1Spec;
impl crate::RegisterSpec for Retcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retcfg1::R`](R) reader structure"]
impl crate::Readable for Retcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`retcfg1::W`](W) writer structure"]
impl crate::Writable for Retcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RETCFG1 to value 0"]
impl crate::Resettable for Retcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
