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
#[doc = "Field `RESERVED0` reader - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATESTCTL_SPEC, u16, u16, 12, O>;
#[doc = "Field `ATEST_RCOSCMF` reader - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
pub type ATEST_RCOSCMF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATEST_RCOSCMF` writer - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
pub type ATEST_RCOSCMF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ATESTCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TEST_RCOSCMF` reader - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
pub type TEST_RCOSCMF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST_RCOSCMF` writer - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
pub type TEST_RCOSCMF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATESTCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED16` reader - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ATESTCTL_SPEC, u16, u16, 15, O>;
#[doc = "Field `SCLK_LF_AUX_EN` reader - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SCLK_LF_AUX_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_LF_AUX_EN` writer - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SCLK_LF_AUX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATESTCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
    #[inline(always)]
    pub fn atest_rcoscmf(&self) -> ATEST_RCOSCMF_R {
        ATEST_RCOSCMF_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
    #[inline(always)]
    pub fn test_rcoscmf(&self) -> TEST_RCOSCMF_R {
        TEST_RCOSCMF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&self) -> SCLK_LF_AUX_EN_R {
        SCLK_LF_AUX_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
    #[inline(always)]
    #[must_use]
    pub fn atest_rcoscmf(&mut self) -> ATEST_RCOSCMF_W<12> {
        ATEST_RCOSCMF_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
    #[inline(always)]
    #[must_use]
    pub fn test_rcoscmf(&mut self) -> TEST_RCOSCMF_W<14> {
        TEST_RCOSCMF_W::new(self)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_aux_en(&mut self) -> SCLK_LF_AUX_EN_W<31> {
        SCLK_LF_AUX_EN_W::new(self)
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
