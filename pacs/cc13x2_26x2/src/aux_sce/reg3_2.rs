#[doc = "Register `REG3_2` reader"]
pub type R = crate::R<Reg3_2Spec>;
#[doc = "Register `REG3_2` writer"]
pub type W = crate::W<Reg3_2Spec>;
#[doc = "Field `REG2` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reg2R = crate::FieldReader<u16>;
#[doc = "Field `REG3` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reg3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg2(&self) -> Reg2R {
        Reg2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg3(&self) -> Reg3R {
        Reg3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg3_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg3_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg3_2Spec;
impl crate::RegisterSpec for Reg3_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg3_2::R`](R) reader structure"]
impl crate::Readable for Reg3_2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg3_2::W`](W) writer structure"]
impl crate::Writable for Reg3_2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG3_2 to value 0"]
impl crate::Resettable for Reg3_2Spec {
    const RESET_VALUE: u32 = 0;
}
