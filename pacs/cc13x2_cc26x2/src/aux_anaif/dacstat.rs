#[doc = "Register `DACSTAT` reader"]
pub struct R(crate::R<DACSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACSTAT` writer"]
pub struct W(crate::W<DACSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACSTAT_SPEC>;
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
impl From<crate::W<DACSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOLD_ACTIVE` reader - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
pub type HOLD_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `HOLD_ACTIVE` writer - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
pub type HOLD_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACSTAT_SPEC, bool, O>;
#[doc = "Field `SETUP_ACTIVE` reader - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
pub type SETUP_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SETUP_ACTIVE` writer - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
pub type SETUP_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACSTAT_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
    #[inline(always)]
    pub fn hold_active(&self) -> HOLD_ACTIVE_R {
        HOLD_ACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
    #[inline(always)]
    pub fn setup_active(&self) -> SETUP_ACTIVE_R {
        SETUP_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
    #[inline(always)]
    #[must_use]
    pub fn hold_active(&mut self) -> HOLD_ACTIVE_W<0> {
        HOLD_ACTIVE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
    #[inline(always)]
    #[must_use]
    pub fn setup_active(&mut self) -> SETUP_ACTIVE_W<1> {
        SETUP_ACTIVE_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacstat](index.html) module"]
pub struct DACSTAT_SPEC;
impl crate::RegisterSpec for DACSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacstat::R](R) reader structure"]
impl crate::Readable for DACSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacstat::W](W) writer structure"]
impl crate::Writable for DACSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACSTAT to value 0"]
impl crate::Resettable for DACSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
