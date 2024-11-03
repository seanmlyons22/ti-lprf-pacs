#[doc = "Register `BUF1` reader"]
pub type R = crate::R<Buf1Spec>;
#[doc = "Register `BUF1` writer"]
pub type W = crate::W<Buf1Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value of BUF\\[63:32\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value of BUF\\[63:32\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of BUF\\[63:32\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of BUF\\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Buf1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Buffer Word 1 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf1Spec;
impl crate::RegisterSpec for Buf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf1::R`](R) reader structure"]
impl crate::Readable for Buf1Spec {}
#[doc = "`write(|w| ..)` method takes [`buf1::W`](W) writer structure"]
impl crate::Writable for Buf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF1 to value 0"]
impl crate::Resettable for Buf1Spec {
    const RESET_VALUE: u32 = 0;
}
