#[doc = "Register `WEPROT_B0_31_0_BY1` reader"]
pub type R = crate::R<WeprotB0_31_0By1Spec>;
#[doc = "Register `WEPROT_B0_31_0_BY1` writer"]
pub type W = crate::W<WeprotB0_31_0By1Spec>;
#[doc = "Field `WEPROT_B0_31_0_BY1` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0_31_0By1R = crate::FieldReader<u32>;
#[doc = "Field `WEPROT_B0_31_0_BY1` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0_31_0By1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_31_0_by1(&self) -> WeprotB0_31_0By1R {
        WeprotB0_31_0By1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_31_0_by1(&mut self) -> WeprotB0_31_0By1W<WeprotB0_31_0By1Spec> {
        WeprotB0_31_0By1W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weprot_b0_31_0_by1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weprot_b0_31_0_by1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeprotB0_31_0By1Spec;
impl crate::RegisterSpec for WeprotB0_31_0By1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weprot_b0_31_0_by1::R`](R) reader structure"]
impl crate::Readable for WeprotB0_31_0By1Spec {}
#[doc = "`write(|w| ..)` method takes [`weprot_b0_31_0_by1::W`](W) writer structure"]
impl crate::Writable for WeprotB0_31_0By1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WEPROT_B0_31_0_BY1 to value 0xffff_ffff"]
impl crate::Resettable for WeprotB0_31_0By1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
