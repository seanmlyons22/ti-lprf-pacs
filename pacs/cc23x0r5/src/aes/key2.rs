#[doc = "Register `KEY2` reader"]
pub type R = crate::R<Key2Spec>;
#[doc = "Register `KEY2` writer"]
pub type W = crate::W<Key2Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value of KEY\\[95:64\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value of KEY\\[95:64\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of KEY\\[95:64\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of KEY\\[95:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Key2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Key Word 2 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2Spec;
impl crate::RegisterSpec for Key2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key2::R`](R) reader structure"]
impl crate::Readable for Key2Spec {}
#[doc = "`write(|w| ..)` method takes [`key2::W`](W) writer structure"]
impl crate::Writable for Key2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY2 to value 0"]
impl crate::Resettable for Key2Spec {
    const RESET_VALUE: u32 = 0;
}
