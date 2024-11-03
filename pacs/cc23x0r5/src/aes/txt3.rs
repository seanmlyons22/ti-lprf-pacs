#[doc = "Register `TXT3` reader"]
pub type R = crate::R<Txt3Spec>;
#[doc = "Register `TXT3` writer"]
pub type W = crate::W<Txt3Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value of TXT\\[127:96\\]
AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value of TXT\\[127:96\\]
AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of TXT\\[127:96\\]
AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of TXT\\[127:96\\]
AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Txt3Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Text Word 3 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txt3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txt3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txt3Spec;
impl crate::RegisterSpec for Txt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txt3::R`](R) reader structure"]
impl crate::Readable for Txt3Spec {}
#[doc = "`write(|w| ..)` method takes [`txt3::W`](W) writer structure"]
impl crate::Writable for Txt3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXT3 to value 0"]
impl crate::Resettable for Txt3Spec {
    const RESET_VALUE: u32 = 0;
}
