#[doc = "Register `JTAGCFG` reader"]
pub struct R(crate::R<JTAGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAGCFG` writer"]
pub struct W(crate::W<JTAGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAGCFG_SPEC>;
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
impl From<crate::W<JTAGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JTAGCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `JTAG_PD_FORCE_ON` reader - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
pub type JTAG_PD_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_PD_FORCE_ON` writer - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
pub type JTAG_PD_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, JTAGCFG_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JTAGCFG_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&self) -> JTAG_PD_FORCE_ON_R {
        JTAG_PD_FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_pd_force_on(&mut self) -> JTAG_PD_FORCE_ON_W<8> {
        JTAG_PD_FORCE_ON_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagcfg](index.html) module"]
pub struct JTAGCFG_SPEC;
impl crate::RegisterSpec for JTAGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtagcfg::R](R) reader structure"]
impl crate::Readable for JTAGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtagcfg::W](W) writer structure"]
impl crate::Writable for JTAGCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JTAGCFG to value 0x0100"]
impl crate::Resettable for JTAGCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
