#[doc = "Register `PDCTL1` reader"]
pub struct R(crate::R<PDCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCTL1` writer"]
pub struct W(crate::W<PDCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCTL1_SPEC>;
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
impl From<crate::W<PDCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCTL1_SPEC, bool, O>;
#[doc = "Field `CPU_ON` reader - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
pub type CPU_ON_R = crate::BitReader<bool>;
#[doc = "Field `CPU_ON` writer - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
pub type CPU_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCTL1_SPEC, bool, O>;
#[doc = "Field `RFC_ON` reader - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
pub type RFC_ON_R = crate::BitReader<bool>;
#[doc = "Field `RFC_ON` writer - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
pub type RFC_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCTL1_SPEC, bool, O>;
#[doc = "Field `VIMS_MODE` reader - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
pub type VIMS_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIMS_MODE` writer - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
pub type VIMS_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDCTL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDCTL1_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[inline(always)]
    pub fn cpu_on(&self) -> CPU_ON_R {
        CPU_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    pub fn vims_mode(&self) -> VIMS_MODE_R {
        VIMS_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_on(&mut self) -> CPU_ON_W<1> {
        CPU_ON_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_on(&mut self) -> RFC_ON_W<2> {
        RFC_ON_W::new(self)
    }
    #[doc = "Bits 3:4 - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    #[must_use]
    pub fn vims_mode(&mut self) -> VIMS_MODE_W<3> {
        VIMS_MODE_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl1](index.html) module"]
pub struct PDCTL1_SPEC;
impl crate::RegisterSpec for PDCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdctl1::R](R) reader structure"]
impl crate::Readable for PDCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdctl1::W](W) writer structure"]
impl crate::Writable for PDCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDCTL1 to value 0x0a"]
impl crate::Resettable for PDCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
