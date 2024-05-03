#[doc = "Register `REG1_0` reader"]
pub type R = crate::R<Reg1_0Spec>;
#[doc = "Register `REG1_0` writer"]
pub type W = crate::W<Reg1_0Spec>;
#[doc = "Field `REG0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reg0R = crate::FieldReader<u16>;
#[doc = "Field `REG0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reg0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG1` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reg1R = crate::FieldReader<u16>;
#[doc = "Field `REG1` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reg1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg0(&self) -> Reg0R {
        Reg0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg1(&self) -> Reg1R {
        Reg1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg0(&mut self) -> Reg0W<Reg1_0Spec> {
        Reg0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg1(&mut self) -> Reg1W<Reg1_0Spec> {
        Reg1W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg1_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg1_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg1_0Spec;
impl crate::RegisterSpec for Reg1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg1_0::R`](R) reader structure"]
impl crate::Readable for Reg1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg1_0::W`](W) writer structure"]
impl crate::Writable for Reg1_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG1_0 to value 0"]
impl crate::Resettable for Reg1_0Spec {
    const RESET_VALUE: u32 = 0;
}
