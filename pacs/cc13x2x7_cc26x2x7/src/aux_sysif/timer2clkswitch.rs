#[doc = "Register `TIMER2CLKSWITCH` reader"]
pub struct R(crate::R<TIMER2CLKSWITCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2CLKSWITCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2CLKSWITCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2CLKSWITCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2CLKSWITCH` writer"]
pub struct W(crate::W<TIMER2CLKSWITCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2CLKSWITCH_SPEC>;
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
impl From<crate::W<TIMER2CLKSWITCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2CLKSWITCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY` reader - 0:0\\]
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - 0:0\\]
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER2CLKSWITCH_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2CLKSWITCH_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
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
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
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
#[doc = "AUX_TIMER2 Clock Switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2clkswitch](index.html) module"]
pub struct TIMER2CLKSWITCH_SPEC;
impl crate::RegisterSpec for TIMER2CLKSWITCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2clkswitch::R](R) reader structure"]
impl crate::Readable for TIMER2CLKSWITCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2clkswitch::W](W) writer structure"]
impl crate::Writable for TIMER2CLKSWITCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2CLKSWITCH to value 0x01"]
impl crate::Resettable for TIMER2CLKSWITCH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
