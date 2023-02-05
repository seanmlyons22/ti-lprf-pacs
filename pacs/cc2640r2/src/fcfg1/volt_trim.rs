#[doc = "Register `VOLT_TRIM` reader"]
pub struct R(crate::R<VOLT_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOLT_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOLT_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOLT_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOLT_TRIM` writer"]
pub struct W(crate::W<VOLT_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOLT_TRIM_SPEC>;
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
impl From<crate::W<VOLT_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOLT_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIMBOD_H` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type TRIMBOD_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMBOD_H` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type TRIMBOD_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED0` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `VDDR_TRIM_SLEEP_H` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_SLEEP_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_TRIM_SLEEP_H` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_SLEEP_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED1` reader - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `VDDR_TRIM_H` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_TRIM_H` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED2` reader - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `VDDR_TRIM_HH` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_HH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_TRIM_HH` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_HH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED3` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VOLT_TRIM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_h(&self) -> TRIMBOD_H_R {
        TRIMBOD_H_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep_h(&self) -> VDDR_TRIM_SLEEP_H_R {
        VDDR_TRIM_SLEEP_H_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_h(&self) -> VDDR_TRIM_H_R {
        VDDR_TRIM_H_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_hh(&self) -> VDDR_TRIM_HH_R {
        VDDR_TRIM_HH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimbod_h(&mut self) -> TRIMBOD_H_W<0> {
        TRIMBOD_H_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<5> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_sleep_h(&mut self) -> VDDR_TRIM_SLEEP_H_W<8> {
        VDDR_TRIM_SLEEP_H_W::new(self)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<13> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_h(&mut self) -> VDDR_TRIM_H_W<16> {
        VDDR_TRIM_H_W::new(self)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<21> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_hh(&mut self) -> VDDR_TRIM_HH_W<24> {
        VDDR_TRIM_HH_W::new(self)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<29> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_trim](index.html) module"]
pub struct VOLT_TRIM_SPEC;
impl crate::RegisterSpec for VOLT_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [volt_trim::R](R) reader structure"]
impl crate::Readable for VOLT_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [volt_trim::W](W) writer structure"]
impl crate::Writable for VOLT_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VOLT_TRIM to value 0xffff_ffe0"]
impl crate::Resettable for VOLT_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffe0;
}
