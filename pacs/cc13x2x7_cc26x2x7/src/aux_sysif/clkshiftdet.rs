#[doc = "Register `CLKSHIFTDET` reader"]
pub struct R(crate::R<CLKSHIFTDET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSHIFTDET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSHIFTDET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSHIFTDET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSHIFTDET` writer"]
pub struct W(crate::W<CLKSHIFTDET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSHIFTDET_SPEC>;
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
impl From<crate::W<CLKSHIFTDET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSHIFTDET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 0:0\\]
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
pub type STAT_R = crate::BitReader<bool>;
#[doc = "Field `STAT` writer - 0:0\\]
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
pub type STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSHIFTDET_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKSHIFTDET_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkshiftdet](index.html) module"]
pub struct CLKSHIFTDET_SPEC;
impl crate::RegisterSpec for CLKSHIFTDET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkshiftdet::R](R) reader structure"]
impl crate::Readable for CLKSHIFTDET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkshiftdet::W](W) writer structure"]
impl crate::Writable for CLKSHIFTDET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSHIFTDET to value 0x01"]
impl crate::Resettable for CLKSHIFTDET_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
