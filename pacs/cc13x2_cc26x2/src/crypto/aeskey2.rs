#[doc = "Register `AESKEY2` reader"]
pub struct R(crate::R<AESKEY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESKEY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESKEY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESKEY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESKEY2` writer"]
pub struct W(crate::W<AESKEY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESKEY2_SPEC>;
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
impl From<crate::W<AESKEY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESKEY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_KEY2` reader - 31:0\\]
AES_KEY2/AES_GHASH_H\\[31:0\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
pub type AES_KEY2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_KEY2` writer - 31:0\\]
AES_KEY2/AES_GHASH_H\\[31:0\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
pub type AES_KEY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESKEY2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AES_KEY2/AES_GHASH_H\\[31:0\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
    #[inline(always)]
    pub fn aes_key2(&self) -> AES_KEY2_R {
        AES_KEY2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AES_KEY2/AES_GHASH_H\\[31:0\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
    #[inline(always)]
    #[must_use]
    pub fn aes_key2(&mut self) -> AES_KEY2_W<0> {
        AES_KEY2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey2](index.html) module"]
pub struct AESKEY2_SPEC;
impl crate::RegisterSpec for AESKEY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aeskey2::R](R) reader structure"]
impl crate::Readable for AESKEY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aeskey2::W](W) writer structure"]
impl crate::Writable for AESKEY2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESKEY2 to value 0"]
impl crate::Resettable for AESKEY2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
