#[doc = "Register `PERBUSDMACLKDIV` reader"]
pub type R = crate::R<PerbusdmaclkdivSpec>;
#[doc = "Register `PERBUSDMACLKDIV` writer"]
pub type W = crate::W<PerbusdmaclkdivSpec>;
#[doc = "Field `SPARE0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Spare0R = crate::FieldReader<u32>;
#[doc = "Field `SPARE0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Spare0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare0(&self) -> Spare0R {
        Spare0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare0(&mut self) -> Spare0W<PerbusdmaclkdivSpec> {
        Spare0W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbusdmaclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbusdmaclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbusdmaclkdivSpec;
impl crate::RegisterSpec for PerbusdmaclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perbusdmaclkdiv::R`](R) reader structure"]
impl crate::Readable for PerbusdmaclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`perbusdmaclkdiv::W`](W) writer structure"]
impl crate::Writable for PerbusdmaclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERBUSDMACLKDIV to value 0"]
impl crate::Resettable for PerbusdmaclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
