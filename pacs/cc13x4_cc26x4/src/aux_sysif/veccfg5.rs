#[doc = "Register `VECCFG5` reader"]
pub struct R(crate::R<VECCFG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VECCFG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VECCFG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VECCFG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VECCFG5` writer"]
pub struct W(crate::W<VECCFG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VECCFG5_SPEC>;
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
impl From<crate::W<VECCFG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VECCFG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VEC_EV` reader - 3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE."]
pub type VEC_EV_R = crate::FieldReader<u8, VEC_EV_A>;
#[doc = "3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VEC_EV_A {
    #[doc = "9: AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY = 9,
    #[doc = "8: WUFLAGS.SW_WU3"]
    SW_WU3 = 8,
    #[doc = "7: WUFLAGS.SW_WU2"]
    SW_WU2 = 7,
    #[doc = "6: WUFLAGS.SW_WU1"]
    SW_WU1 = 6,
    #[doc = "5: WUFLAGS.SW_WU0"]
    SW_WU0 = 5,
    #[doc = "4: WUFLAGS.PROG_WU3"]
    PROG_WU3 = 4,
    #[doc = "3: WUFLAGS.PROG_WU2"]
    PROG_WU2 = 3,
    #[doc = "2: WUFLAGS.PROG_WU1"]
    PROG_WU1 = 2,
    #[doc = "1: WUFLAGS.PROG_WU0"]
    PROG_WU0 = 1,
    #[doc = "0: Vector is disabled."]
    NONE = 0,
}
impl From<VEC_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: VEC_EV_A) -> Self {
        variant as _
    }
}
impl VEC_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VEC_EV_A> {
        match self.bits {
            9 => Some(VEC_EV_A::AON_RTC_CH2_DLY),
            8 => Some(VEC_EV_A::SW_WU3),
            7 => Some(VEC_EV_A::SW_WU2),
            6 => Some(VEC_EV_A::SW_WU1),
            5 => Some(VEC_EV_A::SW_WU0),
            4 => Some(VEC_EV_A::PROG_WU3),
            3 => Some(VEC_EV_A::PROG_WU2),
            2 => Some(VEC_EV_A::PROG_WU1),
            1 => Some(VEC_EV_A::PROG_WU0),
            0 => Some(VEC_EV_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == VEC_EV_A::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `SW_WU3`"]
    #[inline(always)]
    pub fn is_sw_wu3(&self) -> bool {
        *self == VEC_EV_A::SW_WU3
    }
    #[doc = "Checks if the value of the field is `SW_WU2`"]
    #[inline(always)]
    pub fn is_sw_wu2(&self) -> bool {
        *self == VEC_EV_A::SW_WU2
    }
    #[doc = "Checks if the value of the field is `SW_WU1`"]
    #[inline(always)]
    pub fn is_sw_wu1(&self) -> bool {
        *self == VEC_EV_A::SW_WU1
    }
    #[doc = "Checks if the value of the field is `SW_WU0`"]
    #[inline(always)]
    pub fn is_sw_wu0(&self) -> bool {
        *self == VEC_EV_A::SW_WU0
    }
    #[doc = "Checks if the value of the field is `PROG_WU3`"]
    #[inline(always)]
    pub fn is_prog_wu3(&self) -> bool {
        *self == VEC_EV_A::PROG_WU3
    }
    #[doc = "Checks if the value of the field is `PROG_WU2`"]
    #[inline(always)]
    pub fn is_prog_wu2(&self) -> bool {
        *self == VEC_EV_A::PROG_WU2
    }
    #[doc = "Checks if the value of the field is `PROG_WU1`"]
    #[inline(always)]
    pub fn is_prog_wu1(&self) -> bool {
        *self == VEC_EV_A::PROG_WU1
    }
    #[doc = "Checks if the value of the field is `PROG_WU0`"]
    #[inline(always)]
    pub fn is_prog_wu0(&self) -> bool {
        *self == VEC_EV_A::PROG_WU0
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == VEC_EV_A::NONE
    }
}
#[doc = "Field `VEC_EV` writer - 3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE."]
pub type VEC_EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VECCFG5_SPEC, u8, VEC_EV_A, 4, O>;
impl<'a, const O: u8> VEC_EV_W<'a, O> {
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(VEC_EV_A::AON_RTC_CH2_DLY)
    }
    #[doc = "WUFLAGS.SW_WU3"]
    #[inline(always)]
    pub fn sw_wu3(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU3)
    }
    #[doc = "WUFLAGS.SW_WU2"]
    #[inline(always)]
    pub fn sw_wu2(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU2)
    }
    #[doc = "WUFLAGS.SW_WU1"]
    #[inline(always)]
    pub fn sw_wu1(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU1)
    }
    #[doc = "WUFLAGS.SW_WU0"]
    #[inline(always)]
    pub fn sw_wu0(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU0)
    }
    #[doc = "WUFLAGS.PROG_WU3"]
    #[inline(always)]
    pub fn prog_wu3(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU3)
    }
    #[doc = "WUFLAGS.PROG_WU2"]
    #[inline(always)]
    pub fn prog_wu2(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU2)
    }
    #[doc = "WUFLAGS.PROG_WU1"]
    #[inline(always)]
    pub fn prog_wu1(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU1)
    }
    #[doc = "WUFLAGS.PROG_WU0"]
    #[inline(always)]
    pub fn prog_wu0(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU0)
    }
    #[doc = "Vector is disabled."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(VEC_EV_A::NONE)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VECCFG5_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE."]
    #[inline(always)]
    pub fn vec_ev(&self) -> VEC_EV_R {
        VEC_EV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE."]
    #[inline(always)]
    #[must_use]
    pub fn vec_ev(&mut self) -> VEC_EV_W<0> {
        VEC_EV_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vector Configuration 5 AUX_SCE wakeup vector 5 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg5](index.html) module"]
pub struct VECCFG5_SPEC;
impl crate::RegisterSpec for VECCFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [veccfg5::R](R) reader structure"]
impl crate::Readable for VECCFG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [veccfg5::W](W) writer structure"]
impl crate::Writable for VECCFG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VECCFG5 to value 0"]
impl crate::Resettable for VECCFG5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
