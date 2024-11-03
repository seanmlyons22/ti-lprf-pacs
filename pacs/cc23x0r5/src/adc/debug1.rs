#[doc = "Register `DEBUG1` reader"]
pub type R = crate::R<Debug1Spec>;
#[doc = "Register `DEBUG1` writer"]
pub type W = crate::W<Debug1Spec>;
#[doc = "Field `CTRL` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type CtrlR = crate::FieldReader<u32>;
#[doc = "Field `CTRL` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl(&mut self) -> CtrlW<Debug1Spec> {
        CtrlW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Debug1Spec;
impl crate::RegisterSpec for Debug1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug1::R`](R) reader structure"]
impl crate::Readable for Debug1Spec {}
#[doc = "`write(|w| ..)` method takes [`debug1::W`](W) writer structure"]
impl crate::Writable for Debug1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG1 to value 0x0080_1000"]
impl crate::Resettable for Debug1Spec {
    const RESET_VALUE: u32 = 0x0080_1000;
}
