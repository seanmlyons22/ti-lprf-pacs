#[doc = "Register `HASHDIGESTC` reader"]
pub struct R(crate::R<HASHDIGESTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHDIGESTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHDIGESTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHDIGESTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHDIGESTC` writer"]
pub struct W(crate::W<HASHDIGESTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHDIGESTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HASHDIGESTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHDIGESTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH_DIGEST` reader - 31:0\\]
HASH_DIGEST\\[95:64\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub type HASH_DIGEST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HASH_DIGEST` writer - 31:0\\]
HASH_DIGEST\\[95:64\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub type HASH_DIGEST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASHDIGESTC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DIGEST\\[95:64\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&self) -> HASH_DIGEST_R {
        HASH_DIGEST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DIGEST\\[95:64\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    #[must_use]
    pub fn hash_digest(&mut self) -> HASH_DIGEST_W<0> {
        HASH_DIGEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestc](index.html) module"]
pub struct HASHDIGESTC_SPEC;
impl crate::RegisterSpec for HASHDIGESTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashdigestc::R](R) reader structure"]
impl crate::Readable for HASHDIGESTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashdigestc::W](W) writer structure"]
impl crate::Writable for HASHDIGESTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASHDIGESTC to value 0"]
impl crate::Resettable for HASHDIGESTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
