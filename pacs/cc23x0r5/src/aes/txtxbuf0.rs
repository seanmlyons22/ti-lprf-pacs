#[doc = "Register `TXTXBUF0` reader"]
pub type R = crate::R<Txtxbuf0Spec>;
#[doc = "Register `TXTXBUF0` writer"]
pub type W = crate::W<Txtxbuf0Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value read will be TXT0.VAL XOR BUF0.VAL"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value read will be TXT0.VAL XOR BUF0.VAL"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value read will be TXT0.VAL XOR BUF0.VAL"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value read will be TXT0.VAL XOR BUF0.VAL"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Txtxbuf0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Text Word 0 XOR Buffer Word 0 Read this register to obtain plaintext during CFB decryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtxbuf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtxbuf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txtxbuf0Spec;
impl crate::RegisterSpec for Txtxbuf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtxbuf0::R`](R) reader structure"]
impl crate::Readable for Txtxbuf0Spec {}
#[doc = "`write(|w| ..)` method takes [`txtxbuf0::W`](W) writer structure"]
impl crate::Writable for Txtxbuf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXTXBUF0 to value 0"]
impl crate::Resettable for Txtxbuf0Spec {
    const RESET_VALUE: u32 = 0;
}
