#[doc = "Register `HASHMODE` reader"]
pub struct R(crate::R<HASHMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHMODE` writer"]
pub struct W(crate::W<HASHMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHMODE_SPEC>;
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
impl From<crate::W<HASHMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEW_HASH` reader - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
pub type NEW_HASH_R = crate::BitReader<bool>;
#[doc = "Field `NEW_HASH` writer - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
pub type NEW_HASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASHMODE_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 2:1\\]
Write 0s and ignore on reading"]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 2:1\\]
Write 0s and ignore on reading"]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASHMODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `SHA256_MODE` reader - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
pub type SHA256_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SHA256_MODE` writer - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
pub type SHA256_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASHMODE_SPEC, bool, O>;
#[doc = "Field `SHA224_MODE` reader - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
pub type SHA224_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SHA224_MODE` writer - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
pub type SHA224_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASHMODE_SPEC, bool, O>;
#[doc = "Field `SHA512_MODE` reader - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
pub type SHA512_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SHA512_MODE` writer - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
pub type SHA512_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASHMODE_SPEC, bool, O>;
#[doc = "Field `SHA384_MODE` reader - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
pub type SHA384_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SHA384_MODE` writer - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
pub type SHA384_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASHMODE_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Write 0s and ignore on reading"]
pub type RESERVED7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Write 0s and ignore on reading"]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASHMODE_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    pub fn new_hash(&self) -> NEW_HASH_R {
        NEW_HASH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
    #[inline(always)]
    pub fn sha256_mode(&self) -> SHA256_MODE_R {
        SHA256_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
    #[inline(always)]
    pub fn sha224_mode(&self) -> SHA224_MODE_R {
        SHA224_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
    #[inline(always)]
    pub fn sha512_mode(&self) -> SHA512_MODE_R {
        SHA512_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
    #[inline(always)]
    pub fn sha384_mode(&self) -> SHA384_MODE_R {
        SHA384_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    #[must_use]
    pub fn new_hash(&mut self) -> NEW_HASH_W<0> {
        NEW_HASH_W::new(self)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha256_mode(&mut self) -> SHA256_MODE_W<3> {
        SHA256_MODE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha224_mode(&mut self) -> SHA224_MODE_W<4> {
        SHA224_MODE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha512_mode(&mut self) -> SHA512_MODE_W<5> {
        SHA512_MODE_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha384_mode(&mut self) -> SHA384_MODE_W<6> {
        SHA384_MODE_W::new(self)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashmode](index.html) module"]
pub struct HASHMODE_SPEC;
impl crate::RegisterSpec for HASHMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashmode::R](R) reader structure"]
impl crate::Readable for HASHMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashmode::W](W) writer structure"]
impl crate::Writable for HASHMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASHMODE to value 0"]
impl crate::Resettable for HASHMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
