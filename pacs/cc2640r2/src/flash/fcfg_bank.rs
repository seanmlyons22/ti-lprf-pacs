#[doc = "Register `FCFG_BANK` reader"]
pub struct R(crate::R<FCFG_BANK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_BANK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_BANK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_BANK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_BANK` writer"]
pub struct W(crate::W<FCFG_BANK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_BANK_SPEC>;
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
impl From<crate::W<FCFG_BANK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_BANK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAIN_NUM_BANK` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type MAIN_NUM_BANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAIN_NUM_BANK` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type MAIN_NUM_BANK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_BANK_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAIN_BANK_WIDTH` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type MAIN_BANK_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAIN_BANK_WIDTH` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type MAIN_BANK_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_BANK_SPEC, u16, u16, 12, O>;
#[doc = "Field `EE_NUM_BANK` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type EE_NUM_BANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE_NUM_BANK` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type EE_NUM_BANK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_BANK_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE_BANK_WIDTH` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type EE_BANK_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EE_BANK_WIDTH` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type EE_BANK_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_BANK_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_num_bank(&self) -> MAIN_NUM_BANK_R {
        MAIN_NUM_BANK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_bank_width(&self) -> MAIN_BANK_WIDTH_R {
        MAIN_BANK_WIDTH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_num_bank(&self) -> EE_NUM_BANK_R {
        EE_NUM_BANK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_bank_width(&self) -> EE_BANK_WIDTH_R {
        EE_BANK_WIDTH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn main_num_bank(&mut self) -> MAIN_NUM_BANK_W<0> {
        MAIN_NUM_BANK_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn main_bank_width(&mut self) -> MAIN_BANK_WIDTH_W<4> {
        MAIN_BANK_WIDTH_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ee_num_bank(&mut self) -> EE_NUM_BANK_W<16> {
        EE_NUM_BANK_W::new(self)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ee_bank_width(&mut self) -> EE_BANK_WIDTH_W<20> {
        EE_BANK_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_bank](index.html) module"]
pub struct FCFG_BANK_SPEC;
impl crate::RegisterSpec for FCFG_BANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_bank::R](R) reader structure"]
impl crate::Readable for FCFG_BANK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_bank::W](W) writer structure"]
impl crate::Writable for FCFG_BANK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFG_BANK to value 0x0401"]
impl crate::Resettable for FCFG_BANK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0401;
}
