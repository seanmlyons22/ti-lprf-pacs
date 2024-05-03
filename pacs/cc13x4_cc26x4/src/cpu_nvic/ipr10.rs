#[doc = "Register `IPR10` reader"]
pub type R = crate::R<Ipr10Spec>;
#[doc = "Register `IPR10` writer"]
pub type W = crate::W<Ipr10Spec>;
#[doc = "Field `PRI_N0` reader - 7:0\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+0, or is RES0 if the PE does not implement this interrupt"]
pub type PriN0R = crate::FieldReader;
#[doc = "Field `PRI_N0` writer - 7:0\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+0, or is RES0 if the PE does not implement this interrupt"]
pub type PriN0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_N1` reader - 15:8\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+1, or is RES0 if the PE does not implement this interrupt"]
pub type PriN1R = crate::FieldReader;
#[doc = "Field `PRI_N1` writer - 15:8\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+1, or is RES0 if the PE does not implement this interrupt"]
pub type PriN1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_N2` reader - 23:16\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+2, or is RES0 if the PE does not implement this interrupt"]
pub type PriN2R = crate::FieldReader;
#[doc = "Field `PRI_N2` writer - 23:16\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+2, or is RES0 if the PE does not implement this interrupt"]
pub type PriN2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_N3` reader - 31:24\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+3, or is RES0 if the PE does not implement this interrupt"]
pub type PriN3R = crate::FieldReader;
#[doc = "Field `PRI_N3` writer - 31:24\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+3, or is RES0 if the PE does not implement this interrupt"]
pub type PriN3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+0, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n0(&self) -> PriN0R {
        PriN0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+1, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n1(&self) -> PriN1R {
        PriN1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+2, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n2(&self) -> PriN2R {
        PriN2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+3, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n3(&self) -> PriN3R {
        PriN3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+0, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n0(&mut self) -> PriN0W<Ipr10Spec> {
        PriN0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+1, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n1(&mut self) -> PriN1W<Ipr10Spec> {
        PriN1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+2, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n2(&mut self) -> PriN2W<Ipr10Spec> {
        PriN2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
For register NVIC_IPR*10, the priority of interrupt number 4*10+3, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n3(&mut self) -> PriN3W<Ipr10Spec> {
        PriN3W::new(self, 24)
    }
}
#[doc = "Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipr10Spec;
impl crate::RegisterSpec for Ipr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr10::R`](R) reader structure"]
impl crate::Readable for Ipr10Spec {}
#[doc = "`write(|w| ..)` method takes [`ipr10::W`](W) writer structure"]
impl crate::Writable for Ipr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPR10 to value 0"]
impl crate::Resettable for Ipr10Spec {
    const RESET_VALUE: u32 = 0;
}
