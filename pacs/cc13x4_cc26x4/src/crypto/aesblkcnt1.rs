#[doc = "Register `AESBLKCNT1` reader"]
pub struct R(crate::R<AESBLKCNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESBLKCNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESBLKCNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESBLKCNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESBLKCNT1` writer"]
pub struct W(crate::W<AESBLKCNT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESBLKCNT1_SPEC>;
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
impl From<crate::W<AESBLKCNT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESBLKCNT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_BLK_CNT_56_32` reader - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type AES_BLK_CNT_56_32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_BLK_CNT_56_32` writer - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type AES_BLK_CNT_56_32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AESBLKCNT1_SPEC, u32, u32, 25, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESBLKCNT1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:24 - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    pub fn aes_blk_cnt_56_32(&self) -> AES_BLK_CNT_56_32_R {
        AES_BLK_CNT_56_32_R::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:24 - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    #[must_use]
    pub fn aes_blk_cnt_56_32(&mut self) -> AES_BLK_CNT_56_32_W<0> {
        AES_BLK_CNT_56_32_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This counter keeps track of the number of data blocks during AES-CCM and AES-GCM operations. Reading and writing this counter allows interruption and resuming of long CCM or GCM operations. Note that internally, the block counter is used for AAD data as well as encryption/decryption data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesblkcnt1](index.html) module"]
pub struct AESBLKCNT1_SPEC;
impl crate::RegisterSpec for AESBLKCNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesblkcnt1::R](R) reader structure"]
impl crate::Readable for AESBLKCNT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesblkcnt1::W](W) writer structure"]
impl crate::Writable for AESBLKCNT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESBLKCNT1 to value 0"]
impl crate::Resettable for AESBLKCNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
