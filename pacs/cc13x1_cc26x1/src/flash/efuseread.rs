#[doc = "Register `EFUSEREAD` reader"]
pub struct R(crate::R<EFUSEREAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEREAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEREAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEREAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEREAD` writer"]
pub struct W(crate::W<EFUSEREAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEREAD_SPEC>;
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
impl From<crate::W<EFUSEREAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEREAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MARGIN` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type MARGIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MARGIN` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type MARGIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSEREAD_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPARE` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SPARE_R = crate::BitReader<bool>;
#[doc = "Field `SPARE` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEREAD_SPEC, bool, O>;
#[doc = "Field `DEBUG` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DEBUG_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DEBUG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEREAD_SPEC, bool, O>;
#[doc = "Field `READCLOCK` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type READCLOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `READCLOCK` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type READCLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSEREAD_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATABIT` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type DATABIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATABIT` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type DATABIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSEREAD_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEREAD_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn margin(&self) -> MARGIN_R {
        MARGIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn debug(&self) -> DEBUG_R {
        DEBUG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn readclock(&self) -> READCLOCK_R {
        READCLOCK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn databit(&self) -> DATABIT_R {
        DATABIT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn margin(&mut self) -> MARGIN_W<0> {
        MARGIN_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<2> {
        SPARE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn debug(&mut self) -> DEBUG_W<3> {
        DEBUG_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn readclock(&mut self) -> READCLOCK_W<4> {
        READCLOCK_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn databit(&mut self) -> DATABIT_W<8> {
        DATABIT_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseread](index.html) module"]
pub struct EFUSEREAD_SPEC;
impl crate::RegisterSpec for EFUSEREAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseread::R](R) reader structure"]
impl crate::Readable for EFUSEREAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseread::W](W) writer structure"]
impl crate::Writable for EFUSEREAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSEREAD to value 0"]
impl crate::Resettable for EFUSEREAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
