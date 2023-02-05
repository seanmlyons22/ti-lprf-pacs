#[doc = "Register `FCFG_B0_SSIZE0` reader"]
pub struct R(crate::R<FCFG_B0_SSIZE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_B0_SSIZE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_B0_SSIZE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_B0_SSIZE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_B0_SSIZE0` writer"]
pub struct W(crate::W<FCFG_B0_SSIZE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_B0_SSIZE0_SPEC>;
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
impl From<crate::W<FCFG_B0_SSIZE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_B0_SSIZE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0_SECT_SIZE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type B0_SECT_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B0_SECT_SIZE` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type B0_SECT_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_B0_SSIZE0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED4` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_B0_SSIZE0_SPEC, u16, u16, 12, O>;
#[doc = "Field `B0_NUM_SECTORS` reader - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type B0_NUM_SECTORS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `B0_NUM_SECTORS` writer - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type B0_NUM_SECTORS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_B0_SSIZE0_SPEC, u16, u16, 12, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_B0_SSIZE0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_sect_size(&self) -> B0_SECT_SIZE_R {
        B0_SECT_SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_num_sectors(&self) -> B0_NUM_SECTORS_R {
        B0_NUM_SECTORS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_sect_size(&mut self) -> B0_SECT_SIZE_W<0> {
        B0_SECT_SIZE_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_num_sectors(&mut self) -> B0_NUM_SECTORS_W<16> {
        B0_NUM_SECTORS_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> RESERVED28_W<28> {
        RESERVED28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b0_ssize0](index.html) module"]
pub struct FCFG_B0_SSIZE0_SPEC;
impl crate::RegisterSpec for FCFG_B0_SSIZE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_b0_ssize0::R](R) reader structure"]
impl crate::Readable for FCFG_B0_SSIZE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_b0_ssize0::W](W) writer structure"]
impl crate::Writable for FCFG_B0_SSIZE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFG_B0_SSIZE0 to value 0x0020_0004"]
impl crate::Resettable for FCFG_B0_SSIZE0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0004;
}
