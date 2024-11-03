#[doc = "Register `BUF3` reader"]
pub type R = crate::R<Buf3Spec>;
#[doc = "Register `BUF3` writer"]
pub type W = crate::W<Buf3Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value of BUF\\[127:96\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value of BUF\\[127:96\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of BUF\\[127:96\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of BUF\\[127:96\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Buf3Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Buffer Word 3 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes. AUTOCFG.TRGECB decides if a write to this field triggers an encryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf3Spec;
impl crate::RegisterSpec for Buf3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf3::R`](R) reader structure"]
impl crate::Readable for Buf3Spec {}
#[doc = "`write(|w| ..)` method takes [`buf3::W`](W) writer structure"]
impl crate::Writable for Buf3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF3 to value 0"]
impl crate::Resettable for Buf3Spec {
    const RESET_VALUE: u32 = 0;
}
