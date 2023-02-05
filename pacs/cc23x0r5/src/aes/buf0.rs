#[doc = "Register `BUF0` reader"]
pub type R = crate::R<Buf0Spec>;
#[doc = "Register `BUF0` writer"]
pub type W = crate::W<Buf0Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value of BUF\\[31:0\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value of BUF\\[31:0\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of BUF\\[31:0\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of BUF\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Buf0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Buffer Word 0 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf0Spec;
impl crate::RegisterSpec for Buf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf0::R`](R) reader structure"]
impl crate::Readable for Buf0Spec {}
#[doc = "`write(|w| ..)` method takes [`buf0::W`](W) writer structure"]
impl crate::Writable for Buf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF0 to value 0"]
impl crate::Resettable for Buf0Spec {
    const RESET_VALUE: u32 = 0;
}
