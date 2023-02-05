#[doc = "Register `DFTEXECZCTL` reader"]
pub struct R(crate::R<DFTEXECZCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTEXECZCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTEXECZCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTEXECZCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTEXECZCTL` writer"]
pub struct W(crate::W<DFTEXECZCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTEXECZCTL_SPEC>;
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
impl From<crate::W<DFTEXECZCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTEXECZCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXEZOVREN` reader - 0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
pub type EXEZOVREN_R = crate::BitReader<EXEZOVREN_A>;
#[doc = "0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXEZOVREN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<EXEZOVREN_A> for bool {
    #[inline(always)]
    fn from(variant: EXEZOVREN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXEZOVREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXEZOVREN_A {
        match self.bits {
            true => EXEZOVREN_A::ENABLE,
            false => EXEZOVREN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXEZOVREN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXEZOVREN_A::DISABLE
    }
}
#[doc = "Field `EXEZOVREN` writer - 0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
pub type EXEZOVREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTEXECZCTL_SPEC, EXEZOVREN_A, O>;
impl<'a, const O: u8> EXEZOVREN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXEZOVREN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXEZOVREN_A::DISABLE)
    }
}
#[doc = "Field `EXEZ_OVR` reader - 1:1\\]
Override value to be applied to EXECUTEZ"]
pub type EXEZ_OVR_R = crate::BitReader<EXEZ_OVR_A>;
#[doc = "1:1\\]
Override value to be applied to EXECUTEZ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXEZ_OVR_A {
    #[doc = "1: Set EXECUTEZ to 1"]
    ONE = 1,
    #[doc = "0: Set EXECUTEZ to 0"]
    ZERO = 0,
}
impl From<EXEZ_OVR_A> for bool {
    #[inline(always)]
    fn from(variant: EXEZ_OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl EXEZ_OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXEZ_OVR_A {
        match self.bits {
            true => EXEZ_OVR_A::ONE,
            false => EXEZ_OVR_A::ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == EXEZ_OVR_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == EXEZ_OVR_A::ZERO
    }
}
#[doc = "Field `EXEZ_OVR` writer - 1:1\\]
Override value to be applied to EXECUTEZ"]
pub type EXEZ_OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTEXECZCTL_SPEC, EXEZ_OVR_A, O>;
impl<'a, const O: u8> EXEZ_OVR_W<'a, O> {
    #[doc = "Set EXECUTEZ to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(EXEZ_OVR_A::ONE)
    }
    #[doc = "Set EXECUTEZ to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(EXEZ_OVR_A::ZERO)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTEXECZCTL_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
    #[inline(always)]
    pub fn exezovren(&self) -> EXEZOVREN_R {
        EXEZOVREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Override value to be applied to EXECUTEZ"]
    #[inline(always)]
    pub fn exez_ovr(&self) -> EXEZ_OVR_R {
        EXEZ_OVR_R::new(((self.bits >> 1) & 1) != 0)
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
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
    #[inline(always)]
    #[must_use]
    pub fn exezovren(&mut self) -> EXEZOVREN_W<0> {
        EXEZOVREN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Override value to be applied to EXECUTEZ"]
    #[inline(always)]
    #[must_use]
    pub fn exez_ovr(&mut self) -> EXEZ_OVR_W<1> {
        EXEZ_OVR_W::new(self)
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
#[doc = "DFT EXECUTEZ control register. This register allows direct control of the EXECUTEZ to bank and pump for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dftexeczctl](index.html) module"]
pub struct DFTEXECZCTL_SPEC;
impl crate::RegisterSpec for DFTEXECZCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dftexeczctl::R](R) reader structure"]
impl crate::Readable for DFTEXECZCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dftexeczctl::W](W) writer structure"]
impl crate::Writable for DFTEXECZCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTEXECZCTL to value 0x02"]
impl crate::Resettable for DFTEXECZCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
