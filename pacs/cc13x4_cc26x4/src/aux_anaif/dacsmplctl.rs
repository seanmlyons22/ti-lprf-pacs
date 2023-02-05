#[doc = "Register `DACSMPLCTL` reader"]
pub struct R(crate::R<DACSMPLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACSMPLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACSMPLCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACSMPLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACSMPLCTL` writer"]
pub struct W(crate::W<DACSMPLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACSMPLCTL_SPEC>;
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
impl From<crate::W<DACSMPLCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACSMPLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACSMPLCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED7` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DACSMPLCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DAC sample clock enable. 0: Disable sample clock. The sample clock stops low and DACSTAT becomes 0 when the current sample clock period completes. 1: Enable DAC sample clock. DACSTAT must be 0 before you enable sample clock."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<1> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacsmplctl](index.html) module"]
pub struct DACSMPLCTL_SPEC;
impl crate::RegisterSpec for DACSMPLCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacsmplctl::R](R) reader structure"]
impl crate::Readable for DACSMPLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacsmplctl::W](W) writer structure"]
impl crate::Writable for DACSMPLCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACSMPLCTL to value 0"]
impl crate::Resettable for DACSMPLCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
