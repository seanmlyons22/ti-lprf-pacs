#[doc = "Register `FSM_SECTOR` reader"]
pub struct R(crate::R<FSM_SECTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SECTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SECTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SECTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_SECTOR` writer"]
pub struct W(crate::W<FSM_SECTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SECTOR_SPEC>;
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
impl From<crate::W<FSM_SECTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SECTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_OUT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type SEC_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC_OUT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type SEC_OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_SECTOR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECTOR` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECTOR` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SECTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_SECTOR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FSM_SECTOR_EXTENSION` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FSM_SECTOR_EXTENSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSM_SECTOR_EXTENSION` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FSM_SECTOR_EXTENSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_SECTOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SECT_ERASED` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SECT_ERASED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SECT_ERASED` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SECT_ERASED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_SECTOR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_out(&self) -> SEC_OUT_R {
        SEC_OUT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sector(&self) -> SECTOR_R {
        SECTOR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector_extension(&self) -> FSM_SECTOR_EXTENSION_R {
        FSM_SECTOR_EXTENSION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sect_erased(&self) -> SECT_ERASED_R {
        SECT_ERASED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sec_out(&mut self) -> SEC_OUT_W<0> {
        SEC_OUT_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sector(&mut self) -> SECTOR_W<4> {
        SECTOR_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_sector_extension(&mut self) -> FSM_SECTOR_EXTENSION_W<8> {
        FSM_SECTOR_EXTENSION_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sect_erased(&mut self) -> SECT_ERASED_W<16> {
        SECT_ERASED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sector](index.html) module"]
pub struct FSM_SECTOR_SPEC;
impl crate::RegisterSpec for FSM_SECTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_sector::R](R) reader structure"]
impl crate::Readable for FSM_SECTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_sector::W](W) writer structure"]
impl crate::Writable for FSM_SECTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_SECTOR to value 0xffff_0000"]
impl crate::Resettable for FSM_SECTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
