#[doc = "Register `LDO_TRIM` reader"]
pub struct R(crate::R<LDO_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO_TRIM` writer"]
pub struct W(crate::W<LDO_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_TRIM_SPEC>;
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
impl From<crate::W<LDO_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTRIM_DELTA` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_DELTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTRIM_DELTA` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_DELTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED1` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `ITRIM_UDIGLDO` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type ITRIM_UDIGLDO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITRIM_UDIGLDO` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type ITRIM_UDIGLDO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `ITRIM_DIGLDO_LOAD` reader - 12:11\\]
Internal. Only to be used through TI provided API."]
pub type ITRIM_DIGLDO_LOAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITRIM_DIGLDO_LOAD` writer - 12:11\\]
Internal. Only to be used through TI provided API."]
pub type ITRIM_DIGLDO_LOAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED2` reader - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `GLDO_CURSRC` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type GLDO_CURSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GLDO_CURSRC` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type GLDO_CURSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `VDDR_TRIM_SLEEP` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_SLEEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_TRIM_SLEEP` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_SLEEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED4` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_TRIM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_delta(&self) -> VTRIM_DELTA_R {
        VTRIM_DELTA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_udigldo(&self) -> ITRIM_UDIGLDO_R {
        ITRIM_UDIGLDO_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_digldo_load(&self) -> ITRIM_DIGLDO_LOAD_R {
        ITRIM_DIGLDO_LOAD_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldo_cursrc(&self) -> GLDO_CURSRC_R {
        GLDO_CURSRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep(&self) -> VDDR_TRIM_SLEEP_R {
        VDDR_TRIM_SLEEP_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtrim_delta(&mut self) -> VTRIM_DELTA_W<0> {
        VTRIM_DELTA_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<3> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itrim_udigldo(&mut self) -> ITRIM_UDIGLDO_W<8> {
        ITRIM_UDIGLDO_W::new(self)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itrim_digldo_load(&mut self) -> ITRIM_DIGLDO_LOAD_W<11> {
        ITRIM_DIGLDO_LOAD_W::new(self)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<13> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gldo_cursrc(&mut self) -> GLDO_CURSRC_W<16> {
        GLDO_CURSRC_W::new(self)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<19> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_sleep(&mut self) -> VDDR_TRIM_SLEEP_W<24> {
        VDDR_TRIM_SLEEP_W::new(self)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<29> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_trim](index.html) module"]
pub struct LDO_TRIM_SPEC;
impl crate::RegisterSpec for LDO_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo_trim::R](R) reader structure"]
impl crate::Readable for LDO_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo_trim::W](W) writer structure"]
impl crate::Writable for LDO_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDO_TRIM to value 0xe0f8_e0fb"]
impl crate::Resettable for LDO_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0xe0f8_e0fb;
}
