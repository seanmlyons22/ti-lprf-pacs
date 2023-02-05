#[doc = "Register `SIMR` reader"]
pub struct R(crate::R<SIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIMR` writer"]
pub struct W(crate::W<SIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIMR_SPEC>;
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
impl From<crate::W<SIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIM` reader - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
pub type DATAIM_R = crate::BitReader<bool>;
#[doc = "Field `DATAIM` writer - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
pub type DATAIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, O>;
#[doc = "Field `STARTIM` reader - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
pub type STARTIM_R = crate::BitReader<STARTIM_A>;
#[doc = "1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<STARTIM_A> for bool {
    #[inline(always)]
    fn from(variant: STARTIM_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTIM_A {
        match self.bits {
            true => STARTIM_A::EN,
            false => STARTIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STARTIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STARTIM_A::DIS
    }
}
#[doc = "Field `STARTIM` writer - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
pub type STARTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIMR_SPEC, STARTIM_A, O>;
impl<'a, const O: u8> STARTIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STARTIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STARTIM_A::DIS)
    }
}
#[doc = "Field `STOPIM` reader - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
pub type STOPIM_R = crate::BitReader<STOPIM_A>;
#[doc = "2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<STOPIM_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIM_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPIM_A {
        match self.bits {
            true => STOPIM_A::EN,
            false => STOPIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOPIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STOPIM_A::DIS
    }
}
#[doc = "Field `STOPIM` writer - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
pub type STOPIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIMR_SPEC, STOPIM_A, O>;
impl<'a, const O: u8> STOPIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOPIM_A::DIS)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIMR_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&self) -> DATAIM_R {
        DATAIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&self) -> STARTIM_R {
        STARTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&self) -> STOPIM_R {
        STOPIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn dataim(&mut self) -> DATAIM_W<0> {
        DATAIM_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn startim(&mut self) -> STARTIM_W<1> {
        STARTIM_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn stopim(&mut self) -> STOPIM_W<2> {
        STOPIM_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simr](index.html) module"]
pub struct SIMR_SPEC;
impl crate::RegisterSpec for SIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [simr::R](R) reader structure"]
impl crate::Readable for SIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [simr::W](W) writer structure"]
impl crate::Writable for SIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIMR to value 0"]
impl crate::Resettable for SIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
