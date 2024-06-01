#[doc = "Register `AESKEY3` reader"]
pub type R = crate::R<Aeskey3Spec>;
#[doc = "Register `AESKEY3` writer"]
pub type W = crate::W<Aeskey3Spec>;
#[doc = "Field `AES_KEY3` reader - 31:0\\]
AES_KEY3\\[31:0\\]/AES_KEY2\\[159:128\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
pub type AesKey3R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEY3` writer - 31:0\\]
AES_KEY3\\[31:0\\]/AES_KEY2\\[159:128\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
pub type AesKey3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AES_KEY3\\[31:0\\]/AES_KEY2\\[159:128\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
    #[inline(always)]
    pub fn aes_key3(&self) -> AesKey3R {
        AesKey3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AES_KEY3\\[31:0\\]/AES_KEY2\\[159:128\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
    #[inline(always)]
    #[must_use]
    pub fn aes_key3(&mut self) -> AesKey3W<Aeskey3Spec> {
        AesKey3W::new(self, 0)
    }
}
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeskey3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeskey3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey3Spec;
impl crate::RegisterSpec for Aeskey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey3::R`](R) reader structure"]
impl crate::Readable for Aeskey3Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey3::W`](W) writer structure"]
impl crate::Writable for Aeskey3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY3 to value 0"]
impl crate::Resettable for Aeskey3Spec {
    const RESET_VALUE: u32 = 0;
}
