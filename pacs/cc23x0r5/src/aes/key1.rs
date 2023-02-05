#[doc = "Register `KEY1` reader"]
pub type R = crate::R<Key1Spec>;
#[doc = "Register `KEY1` writer"]
pub type W = crate::W<Key1Spec>;
#[doc = "Field `VAL` writer - 31:0\\]
Value of KEY\\[63:32\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of KEY\\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Key1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Key Word 1 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1Spec;
impl crate::RegisterSpec for Key1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1::R`](R) reader structure"]
impl crate::Readable for Key1Spec {}
#[doc = "`write(|w| ..)` method takes [`key1::W`](W) writer structure"]
impl crate::Writable for Key1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for Key1Spec {
    const RESET_VALUE: u32 = 0;
}
