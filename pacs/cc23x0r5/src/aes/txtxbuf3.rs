#[doc = "Register `TXTXBUF3` reader"]
pub type R = crate::R<Txtxbuf3Spec>;
#[doc = "Register `TXTXBUF3` writer"]
pub type W = crate::W<Txtxbuf3Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value read will be TXT3.VAL XOR BUF3.VAL"]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value read will be TXT3.VAL XOR BUF3.VAL"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {}
#[doc = "Text Word 3 XOR Buffer Word3 Read this register to obtain plaintext during CFB decryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtxbuf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtxbuf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txtxbuf3Spec;
impl crate::RegisterSpec for Txtxbuf3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtxbuf3::R`](R) reader structure"]
impl crate::Readable for Txtxbuf3Spec {}
#[doc = "`write(|w| ..)` method takes [`txtxbuf3::W`](W) writer structure"]
impl crate::Writable for Txtxbuf3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXTXBUF3 to value 0"]
impl crate::Resettable for Txtxbuf3Spec {
    const RESET_VALUE: u32 = 0;
}
