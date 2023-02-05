#[doc = "Register `EFUSEPROGRAM` reader"]
pub struct R(crate::R<EFUSEPROGRAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEPROGRAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEPROGRAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEPROGRAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEPROGRAM` writer"]
pub struct W(crate::W<EFUSEPROGRAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEPROGRAM_SPEC>;
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
impl From<crate::W<EFUSEPROGRAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEPROGRAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITECLOCK` reader - 8:0\\]
Internal. Only to be used through TI provided API."]
pub type WRITECLOCK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WRITECLOCK` writer - 8:0\\]
Internal. Only to be used through TI provided API."]
pub type WRITECLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEPROGRAM_SPEC, u16, u16, 9, O>;
#[doc = "Field `ITERATIONS` reader - 12:9\\]
Internal. Only to be used through TI provided API."]
pub type ITERATIONS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITERATIONS` writer - 12:9\\]
Internal. Only to be used through TI provided API."]
pub type ITERATIONS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEPROGRAM_SPEC, u8, u8, 4, O>;
#[doc = "Field `VPPTOVDD` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type VPPTOVDD_R = crate::BitReader<bool>;
#[doc = "Field `VPPTOVDD` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type VPPTOVDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEPROGRAM_SPEC, bool, O>;
#[doc = "Field `CLOCKSTALL` reader - 29:14\\]
Internal. Only to be used through TI provided API."]
pub type CLOCKSTALL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLOCKSTALL` writer - 29:14\\]
Internal. Only to be used through TI provided API."]
pub type CLOCKSTALL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEPROGRAM_SPEC, u16, u16, 16, O>;
#[doc = "Field `COMPAREDISABLE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type COMPAREDISABLE_R = crate::BitReader<bool>;
#[doc = "Field `COMPAREDISABLE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type COMPAREDISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEPROGRAM_SPEC, bool, O>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED31_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED31_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEPROGRAM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn writeclock(&self) -> WRITECLOCK_R {
        WRITECLOCK_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iterations(&self) -> ITERATIONS_R {
        ITERATIONS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vpptovdd(&self) -> VPPTOVDD_R {
        VPPTOVDD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clockstall(&self) -> CLOCKSTALL_R {
        CLOCKSTALL_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comparedisable(&self) -> COMPAREDISABLE_R {
        COMPAREDISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn writeclock(&mut self) -> WRITECLOCK_W<0> {
        WRITECLOCK_W::new(self)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iterations(&mut self) -> ITERATIONS_W<9> {
        ITERATIONS_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vpptovdd(&mut self) -> VPPTOVDD_W<13> {
        VPPTOVDD_W::new(self)
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn clockstall(&mut self) -> CLOCKSTALL_W<14> {
        CLOCKSTALL_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn comparedisable(&mut self) -> COMPAREDISABLE_W<30> {
        COMPAREDISABLE_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> RESERVED31_W<31> {
        RESERVED31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseprogram](index.html) module"]
pub struct EFUSEPROGRAM_SPEC;
impl crate::RegisterSpec for EFUSEPROGRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseprogram::R](R) reader structure"]
impl crate::Readable for EFUSEPROGRAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseprogram::W](W) writer structure"]
impl crate::Writable for EFUSEPROGRAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSEPROGRAM to value 0"]
impl crate::Resettable for EFUSEPROGRAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
