#[doc = "Register `HASHDIGESTD` reader"]
pub type R = crate::R<HashdigestdSpec>;
#[doc = "Register `HASHDIGESTD` writer"]
pub type W = crate::W<HashdigestdSpec>;
#[doc = "Field `HASH_DIGEST` reader - 31:0\\]
HASH_DIGEST\\[127:96\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub type HashDigestR = crate::FieldReader<u32>;
#[doc = "Field `HASH_DIGEST` writer - 31:0\\]
HASH_DIGEST\\[127:96\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub type HashDigestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DIGEST\\[127:96\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&self) -> HashDigestR {
        HashDigestR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DIGEST\\[127:96\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    #[must_use]
    pub fn hash_digest(&mut self) -> HashDigestW<HashdigestdSpec> {
        HashDigestW::new(self, 0)
    }
}
#[doc = "HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashdigestdSpec;
impl crate::RegisterSpec for HashdigestdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashdigestd::R`](R) reader structure"]
impl crate::Readable for HashdigestdSpec {}
#[doc = "`write(|w| ..)` method takes [`hashdigestd::W`](W) writer structure"]
impl crate::Writable for HashdigestdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHDIGESTD to value 0"]
impl crate::Resettable for HashdigestdSpec {
    const RESET_VALUE: u32 = 0;
}