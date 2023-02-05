#[doc = "Register `ATESTCTL` reader"]
pub struct R(crate::R<ATESTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATESTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATESTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATESTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATESTCTL` writer"]
pub struct W(crate::W<ATESTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATESTCTL_SPEC>;
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
impl From<crate::W<ATESTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATESTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED0` writer - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATESTCTL_SPEC, u32, u32, 29, O>;
#[doc = "Field `SCLK_LF_AUX_EN` reader - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SCLK_LF_AUX_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_LF_AUX_EN` writer - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SCLK_LF_AUX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATESTCTL_SPEC, bool, O>;
#[doc = "Field `SPARE30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATESTCTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&self) -> SCLK_LF_AUX_EN_R {
        SCLK_LF_AUX_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare30(&self) -> SPARE30_R {
        SPARE30_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_aux_en(&mut self) -> SCLK_LF_AUX_EN_W<29> {
        SCLK_LF_AUX_EN_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare30(&mut self) -> SPARE30_W<30> {
        SPARE30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Test Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atestctl](index.html) module"]
pub struct ATESTCTL_SPEC;
impl crate::RegisterSpec for ATESTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atestctl::R](R) reader structure"]
impl crate::Readable for ATESTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atestctl::W](W) writer structure"]
impl crate::Writable for ATESTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATESTCTL to value 0"]
impl crate::Resettable for ATESTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
