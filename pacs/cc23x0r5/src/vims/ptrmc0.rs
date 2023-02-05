#[doc = "Register `PTRMC0` reader"]
pub type R = crate::R<Ptrmc0Spec>;
#[doc = "Register `PTRMC0` writer"]
pub type W = crate::W<Ptrmc0Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Flash charge pump trim value."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Flash charge pump trim value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Flash charge pump trim value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Flash charge pump trim value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Ptrmc0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Stores FLASH Pump trim values. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptrmc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptrmc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ptrmc0Spec;
impl crate::RegisterSpec for Ptrmc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptrmc0::R`](R) reader structure"]
impl crate::Readable for Ptrmc0Spec {}
#[doc = "`write(|w| ..)` method takes [`ptrmc0::W`](W) writer structure"]
impl crate::Writable for Ptrmc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTRMC0 to value 0x131a_0000"]
impl crate::Resettable for Ptrmc0Spec {
    const RESET_VALUE: u32 = 0x131a_0000;
}
