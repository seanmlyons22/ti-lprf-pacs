#[doc = "Register `REG7_6` reader"]
pub type R = crate::R<Reg7_6Spec>;
#[doc = "Register `REG7_6` writer"]
pub type W = crate::W<Reg7_6Spec>;
#[doc = "Field `REG6` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reg6R = crate::FieldReader<u16>;
#[doc = "Field `REG7` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reg7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg6(&self) -> Reg6R {
        Reg6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg7(&self) -> Reg7R {
        Reg7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg7_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg7_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg7_6Spec;
impl crate::RegisterSpec for Reg7_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg7_6::R`](R) reader structure"]
impl crate::Readable for Reg7_6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg7_6::W`](W) writer structure"]
impl crate::Writable for Reg7_6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG7_6 to value 0"]
impl crate::Resettable for Reg7_6Spec {
    const RESET_VALUE: u32 = 0;
}
