#[doc = "Register `EVT_MODE` reader"]
pub struct R(crate::R<EVT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVT_MODE` writer"]
pub struct W(crate::W<EVT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVT_MODE_SPEC>;
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
impl From<crate::W<EVT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0_CFG` reader - 1:0\\]
Event line mode select for event corresponding to IPSTANDARD.INT_EVENT0"]
pub type INT0_CFG_R = crate::FieldReader<u8, INT0_CFG_A>;
#[doc = "1:0\\]
Event line mode select for event corresponding to IPSTANDARD.INT_EVENT0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INT0_CFG_A {
    #[doc = "2: The interrupt or event line is in hardware mode. The hardware automatically clears the RIS flag."]
    HARDWARE = 2,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    SOFTWARE = 1,
    #[doc = "0: The interrupt or event line is disabled."]
    DISABLE = 0,
}
impl From<INT0_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: INT0_CFG_A) -> Self {
        variant as _
    }
}
impl INT0_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INT0_CFG_A> {
        match self.bits {
            2 => Some(INT0_CFG_A::HARDWARE),
            1 => Some(INT0_CFG_A::SOFTWARE),
            0 => Some(INT0_CFG_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == INT0_CFG_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == INT0_CFG_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INT0_CFG_A::DISABLE
    }
}
#[doc = "Field `INT0_CFG` writer - 1:0\\]
Event line mode select for event corresponding to IPSTANDARD.INT_EVENT0"]
pub type INT0_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVT_MODE_SPEC, u8, INT0_CFG_A, 2, O>;
impl<'a, const O: u8> INT0_CFG_W<'a, O> {
    #[doc = "The interrupt or event line is in hardware mode. The hardware automatically clears the RIS flag."]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(INT0_CFG_A::HARDWARE)
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(INT0_CFG_A::SOFTWARE)
    }
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INT0_CFG_A::DISABLE)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVT_MODE_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Event line mode select for event corresponding to IPSTANDARD.INT_EVENT0"]
    #[inline(always)]
    pub fn int0_cfg(&self) -> INT0_CFG_R {
        INT0_CFG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Event line mode select for event corresponding to IPSTANDARD.INT_EVENT0"]
    #[inline(always)]
    #[must_use]
    pub fn int0_cfg(&mut self) -> INT0_CFG_W<0> {
        INT0_CFG_W::new(self)
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
#[doc = "Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS) Note: The recommendation is to use SPI in the software mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evt_mode](index.html) module"]
pub struct EVT_MODE_SPEC;
impl crate::RegisterSpec for EVT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evt_mode::R](R) reader structure"]
impl crate::Readable for EVT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evt_mode::W](W) writer structure"]
impl crate::Writable for EVT_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVT_MODE to value 0x01"]
impl crate::Resettable for EVT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
