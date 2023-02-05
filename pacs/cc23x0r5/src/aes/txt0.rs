#[doc = "Register `TXT0` reader"]
pub type R = crate::R<Txt0Spec>;
#[doc = "Register `TXT0` writer"]
pub type W = crate::W<Txt0Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value of TXT\\[31:0\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value of TXT\\[31:0\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of TXT\\[31:0\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of TXT\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Txt0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Text Word 0 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txt0Spec;
impl crate::RegisterSpec for Txt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txt0::R`](R) reader structure"]
impl crate::Readable for Txt0Spec {}
#[doc = "`write(|w| ..)` method takes [`txt0::W`](W) writer structure"]
impl crate::Writable for Txt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXT0 to value 0"]
impl crate::Resettable for Txt0Spec {
    const RESET_VALUE: u32 = 0;
}
