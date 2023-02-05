#[doc = "Register `SYSGPOCTL` reader"]
pub struct R(crate::R<SYSGPOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSGPOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSGPOCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSGPOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSGPOCTL` writer"]
pub struct W(crate::W<SYSGPOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSGPOCTL_SPEC>;
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
impl From<crate::W<SYSGPOCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSGPOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOCTL0` reader - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
pub type GPOCTL0_R = crate::FieldReader<u8, GPOCTL0_A>;
#[doc = "3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOCTL0_A {
    #[doc = "15: RAT GPO line 3"]
    RATGPO3 = 15,
    #[doc = "14: RAT GPO line 2"]
    RATGPO2 = 14,
    #[doc = "13: RAT GPO line 1"]
    RATGPO1 = 13,
    #[doc = "12: RAT GPO line 0"]
    RATGPO0 = 12,
    #[doc = "11: RFE GPO line 3"]
    RFEGPO3 = 11,
    #[doc = "10: RFE GPO line 2"]
    RFEGPO2 = 10,
    #[doc = "9: RFE GPO line 1"]
    RFEGPO1 = 9,
    #[doc = "8: RFE GPO line 0"]
    RFEGPO0 = 8,
    #[doc = "7: MCE GPO line 3"]
    MCEGPO3 = 7,
    #[doc = "6: MCE GPO line 2"]
    MCEGPO2 = 6,
    #[doc = "5: MCE GPO line 1"]
    MCEGPO1 = 5,
    #[doc = "4: MCE GPO line 0"]
    MCEGPO0 = 4,
    #[doc = "3: CPE GPO line 3"]
    CPEGPO3 = 3,
    #[doc = "2: CPE GPO line 2"]
    CPEGPO2 = 2,
    #[doc = "1: CPE GPO line 1"]
    CPEGPO1 = 1,
    #[doc = "0: CPE GPO line 0"]
    CPEGPO0 = 0,
}
impl From<GPOCTL0_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOCTL0_A) -> Self {
        variant as _
    }
}
impl GPOCTL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOCTL0_A {
        match self.bits {
            15 => GPOCTL0_A::RATGPO3,
            14 => GPOCTL0_A::RATGPO2,
            13 => GPOCTL0_A::RATGPO1,
            12 => GPOCTL0_A::RATGPO0,
            11 => GPOCTL0_A::RFEGPO3,
            10 => GPOCTL0_A::RFEGPO2,
            9 => GPOCTL0_A::RFEGPO1,
            8 => GPOCTL0_A::RFEGPO0,
            7 => GPOCTL0_A::MCEGPO3,
            6 => GPOCTL0_A::MCEGPO2,
            5 => GPOCTL0_A::MCEGPO1,
            4 => GPOCTL0_A::MCEGPO0,
            3 => GPOCTL0_A::CPEGPO3,
            2 => GPOCTL0_A::CPEGPO2,
            1 => GPOCTL0_A::CPEGPO1,
            0 => GPOCTL0_A::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL0_A::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL0_A::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL0_A::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL0_A::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL0_A::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL0_A::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL0_A::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL0_A::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL0_A::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL0_A::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL0_A::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL0_A::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL0_A::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL0_A::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL0_A::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL0_A::CPEGPO0
    }
}
#[doc = "Field `GPOCTL0` writer - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
pub type GPOCTL0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SYSGPOCTL_SPEC, u8, GPOCTL0_A, 4, O>;
impl<'a, const O: u8> GPOCTL0_W<'a, O> {
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL0_A::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL0_A::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL0_A::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL0_A::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL0_A::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL0_A::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL0_A::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL0_A::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL0_A::CPEGPO0)
    }
}
#[doc = "Field `GPOCTL1` reader - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
pub type GPOCTL1_R = crate::FieldReader<u8, GPOCTL1_A>;
#[doc = "7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOCTL1_A {
    #[doc = "15: RAT GPO line 3"]
    RATGPO3 = 15,
    #[doc = "14: RAT GPO line 2"]
    RATGPO2 = 14,
    #[doc = "13: RAT GPO line 1"]
    RATGPO1 = 13,
    #[doc = "12: RAT GPO line 0"]
    RATGPO0 = 12,
    #[doc = "11: RFE GPO line 3"]
    RFEGPO3 = 11,
    #[doc = "10: RFE GPO line 2"]
    RFEGPO2 = 10,
    #[doc = "9: RFE GPO line 1"]
    RFEGPO1 = 9,
    #[doc = "8: RFE GPO line 0"]
    RFEGPO0 = 8,
    #[doc = "7: MCE GPO line 3"]
    MCEGPO3 = 7,
    #[doc = "6: MCE GPO line 2"]
    MCEGPO2 = 6,
    #[doc = "5: MCE GPO line 1"]
    MCEGPO1 = 5,
    #[doc = "4: MCE GPO line 0"]
    MCEGPO0 = 4,
    #[doc = "3: CPE GPO line 3"]
    CPEGPO3 = 3,
    #[doc = "2: CPE GPO line 2"]
    CPEGPO2 = 2,
    #[doc = "1: CPE GPO line 1"]
    CPEGPO1 = 1,
    #[doc = "0: CPE GPO line 0"]
    CPEGPO0 = 0,
}
impl From<GPOCTL1_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOCTL1_A) -> Self {
        variant as _
    }
}
impl GPOCTL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOCTL1_A {
        match self.bits {
            15 => GPOCTL1_A::RATGPO3,
            14 => GPOCTL1_A::RATGPO2,
            13 => GPOCTL1_A::RATGPO1,
            12 => GPOCTL1_A::RATGPO0,
            11 => GPOCTL1_A::RFEGPO3,
            10 => GPOCTL1_A::RFEGPO2,
            9 => GPOCTL1_A::RFEGPO1,
            8 => GPOCTL1_A::RFEGPO0,
            7 => GPOCTL1_A::MCEGPO3,
            6 => GPOCTL1_A::MCEGPO2,
            5 => GPOCTL1_A::MCEGPO1,
            4 => GPOCTL1_A::MCEGPO0,
            3 => GPOCTL1_A::CPEGPO3,
            2 => GPOCTL1_A::CPEGPO2,
            1 => GPOCTL1_A::CPEGPO1,
            0 => GPOCTL1_A::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL1_A::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL1_A::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL1_A::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL1_A::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL1_A::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL1_A::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL1_A::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL1_A::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL1_A::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL1_A::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL1_A::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL1_A::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL1_A::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL1_A::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL1_A::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL1_A::CPEGPO0
    }
}
#[doc = "Field `GPOCTL1` writer - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
pub type GPOCTL1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SYSGPOCTL_SPEC, u8, GPOCTL1_A, 4, O>;
impl<'a, const O: u8> GPOCTL1_W<'a, O> {
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL1_A::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL1_A::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL1_A::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL1_A::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL1_A::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL1_A::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL1_A::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL1_A::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL1_A::CPEGPO0)
    }
}
#[doc = "Field `GPOCTL2` reader - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
pub type GPOCTL2_R = crate::FieldReader<u8, GPOCTL2_A>;
#[doc = "11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOCTL2_A {
    #[doc = "15: RAT GPO line 3"]
    RATGPO3 = 15,
    #[doc = "14: RAT GPO line 2"]
    RATGPO2 = 14,
    #[doc = "13: RAT GPO line 1"]
    RATGPO1 = 13,
    #[doc = "12: RAT GPO line 0"]
    RATGPO0 = 12,
    #[doc = "11: RFE GPO line 3"]
    RFEGPO3 = 11,
    #[doc = "10: RFE GPO line 2"]
    RFEGPO2 = 10,
    #[doc = "9: RFE GPO line 1"]
    RFEGPO1 = 9,
    #[doc = "8: RFE GPO line 0"]
    RFEGPO0 = 8,
    #[doc = "7: MCE GPO line 3"]
    MCEGPO3 = 7,
    #[doc = "6: MCE GPO line 2"]
    MCEGPO2 = 6,
    #[doc = "5: MCE GPO line 1"]
    MCEGPO1 = 5,
    #[doc = "4: MCE GPO line 0"]
    MCEGPO0 = 4,
    #[doc = "3: CPE GPO line 3"]
    CPEGPO3 = 3,
    #[doc = "2: CPE GPO line 2"]
    CPEGPO2 = 2,
    #[doc = "1: CPE GPO line 1"]
    CPEGPO1 = 1,
    #[doc = "0: CPE GPO line 0"]
    CPEGPO0 = 0,
}
impl From<GPOCTL2_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOCTL2_A) -> Self {
        variant as _
    }
}
impl GPOCTL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOCTL2_A {
        match self.bits {
            15 => GPOCTL2_A::RATGPO3,
            14 => GPOCTL2_A::RATGPO2,
            13 => GPOCTL2_A::RATGPO1,
            12 => GPOCTL2_A::RATGPO0,
            11 => GPOCTL2_A::RFEGPO3,
            10 => GPOCTL2_A::RFEGPO2,
            9 => GPOCTL2_A::RFEGPO1,
            8 => GPOCTL2_A::RFEGPO0,
            7 => GPOCTL2_A::MCEGPO3,
            6 => GPOCTL2_A::MCEGPO2,
            5 => GPOCTL2_A::MCEGPO1,
            4 => GPOCTL2_A::MCEGPO0,
            3 => GPOCTL2_A::CPEGPO3,
            2 => GPOCTL2_A::CPEGPO2,
            1 => GPOCTL2_A::CPEGPO1,
            0 => GPOCTL2_A::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL2_A::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL2_A::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL2_A::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL2_A::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL2_A::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL2_A::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL2_A::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL2_A::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL2_A::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL2_A::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL2_A::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL2_A::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL2_A::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL2_A::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL2_A::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL2_A::CPEGPO0
    }
}
#[doc = "Field `GPOCTL2` writer - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
pub type GPOCTL2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SYSGPOCTL_SPEC, u8, GPOCTL2_A, 4, O>;
impl<'a, const O: u8> GPOCTL2_W<'a, O> {
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL2_A::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL2_A::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL2_A::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL2_A::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL2_A::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL2_A::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL2_A::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL2_A::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL2_A::CPEGPO0)
    }
}
#[doc = "Field `GPOCTL3` reader - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
pub type GPOCTL3_R = crate::FieldReader<u8, GPOCTL3_A>;
#[doc = "15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOCTL3_A {
    #[doc = "15: RAT GPO line 3"]
    RATGPO3 = 15,
    #[doc = "14: RAT GPO line 2"]
    RATGPO2 = 14,
    #[doc = "13: RAT GPO line 1"]
    RATGPO1 = 13,
    #[doc = "12: RAT GPO line 0"]
    RATGPO0 = 12,
    #[doc = "11: RFE GPO line 3"]
    RFEGPO3 = 11,
    #[doc = "10: RFE GPO line 2"]
    RFEGPO2 = 10,
    #[doc = "9: RFE GPO line 1"]
    RFEGPO1 = 9,
    #[doc = "8: RFE GPO line 0"]
    RFEGPO0 = 8,
    #[doc = "7: MCE GPO line 3"]
    MCEGPO3 = 7,
    #[doc = "6: MCE GPO line 2"]
    MCEGPO2 = 6,
    #[doc = "5: MCE GPO line 1"]
    MCEGPO1 = 5,
    #[doc = "4: MCE GPO line 0"]
    MCEGPO0 = 4,
    #[doc = "3: CPE GPO line 3"]
    CPEGPO3 = 3,
    #[doc = "2: CPE GPO line 2"]
    CPEGPO2 = 2,
    #[doc = "1: CPE GPO line 1"]
    CPEGPO1 = 1,
    #[doc = "0: CPE GPO line 0"]
    CPEGPO0 = 0,
}
impl From<GPOCTL3_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOCTL3_A) -> Self {
        variant as _
    }
}
impl GPOCTL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOCTL3_A {
        match self.bits {
            15 => GPOCTL3_A::RATGPO3,
            14 => GPOCTL3_A::RATGPO2,
            13 => GPOCTL3_A::RATGPO1,
            12 => GPOCTL3_A::RATGPO0,
            11 => GPOCTL3_A::RFEGPO3,
            10 => GPOCTL3_A::RFEGPO2,
            9 => GPOCTL3_A::RFEGPO1,
            8 => GPOCTL3_A::RFEGPO0,
            7 => GPOCTL3_A::MCEGPO3,
            6 => GPOCTL3_A::MCEGPO2,
            5 => GPOCTL3_A::MCEGPO1,
            4 => GPOCTL3_A::MCEGPO0,
            3 => GPOCTL3_A::CPEGPO3,
            2 => GPOCTL3_A::CPEGPO2,
            1 => GPOCTL3_A::CPEGPO1,
            0 => GPOCTL3_A::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL3_A::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL3_A::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL3_A::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL3_A::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL3_A::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL3_A::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL3_A::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL3_A::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL3_A::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL3_A::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL3_A::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL3_A::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL3_A::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL3_A::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL3_A::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL3_A::CPEGPO0
    }
}
#[doc = "Field `GPOCTL3` writer - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
pub type GPOCTL3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SYSGPOCTL_SPEC, u8, GPOCTL3_A, 4, O>;
impl<'a, const O: u8> GPOCTL3_W<'a, O> {
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL3_A::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL3_A::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL3_A::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL3_A::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL3_A::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL3_A::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL3_A::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL3_A::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL3_A::CPEGPO0)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSGPOCTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[inline(always)]
    pub fn gpoctl0(&self) -> GPOCTL0_R {
        GPOCTL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[inline(always)]
    pub fn gpoctl1(&self) -> GPOCTL1_R {
        GPOCTL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[inline(always)]
    pub fn gpoctl2(&self) -> GPOCTL2_R {
        GPOCTL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[inline(always)]
    pub fn gpoctl3(&self) -> GPOCTL3_R {
        GPOCTL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl0(&mut self) -> GPOCTL0_W<0> {
        GPOCTL0_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl1(&mut self) -> GPOCTL1_W<4> {
        GPOCTL1_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl2(&mut self) -> GPOCTL2_W<8> {
        GPOCTL2_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl3(&mut self) -> GPOCTL3_W<12> {
        GPOCTL3_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF Core General Purpose Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysgpoctl](index.html) module"]
pub struct SYSGPOCTL_SPEC;
impl crate::RegisterSpec for SYSGPOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysgpoctl::R](R) reader structure"]
impl crate::Readable for SYSGPOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysgpoctl::W](W) writer structure"]
impl crate::Writable for SYSGPOCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSGPOCTL to value 0"]
impl crate::Resettable for SYSGPOCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
