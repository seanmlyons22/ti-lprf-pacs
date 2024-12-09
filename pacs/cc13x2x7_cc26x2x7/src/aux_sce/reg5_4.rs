#[doc = "Register `REG5_4` reader"]
pub type R = crate::R<Reg5_4Spec>;
#[doc = "Register `REG5_4` writer"]
pub type W = crate::W<Reg5_4Spec>;
#[doc = "Field `REG4` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reg4R = crate::FieldReader<u16>;
#[doc = "Field `REG5` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reg5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg4(&self) -> Reg4R {
        Reg4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg5(&self) -> Reg5R {
        Reg5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg5_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg5_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg5_4Spec;
impl crate::RegisterSpec for Reg5_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg5_4::R`](R) reader structure"]
impl crate::Readable for Reg5_4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg5_4::W`](W) writer structure"]
impl crate::Writable for Reg5_4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG5_4 to value 0"]
impl crate::Resettable for Reg5_4Spec {
    const RESET_VALUE: u32 = 0;
}
