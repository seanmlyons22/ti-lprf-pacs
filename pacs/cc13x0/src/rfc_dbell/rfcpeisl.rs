#[doc = "Register `RFCPEISL` reader"]
pub struct R(crate::R<RFCPEISL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCPEISL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCPEISL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCPEISL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCPEISL` writer"]
pub struct W(crate::W<RFCPEISL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCPEISL_SPEC>;
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
impl From<crate::W<RFCPEISL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCPEISL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND_DONE` reader - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
pub type COMMAND_DONE_R = crate::BitReader<COMMAND_DONE_A>;
#[doc = "0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl COMMAND_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMMAND_DONE_A {
        match self.bits {
            true => COMMAND_DONE_A::CPE1,
            false => COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == COMMAND_DONE_A::CPE0
    }
}
#[doc = "Field `COMMAND_DONE` writer - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
pub type COMMAND_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, COMMAND_DONE_A, O>;
impl<'a, const O: u8> COMMAND_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(COMMAND_DONE_A::CPE0)
    }
}
#[doc = "Field `LAST_COMMAND_DONE` reader - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
pub type LAST_COMMAND_DONE_R = crate::BitReader<LAST_COMMAND_DONE_A>;
#[doc = "1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAST_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<LAST_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl LAST_COMMAND_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_COMMAND_DONE_A {
        match self.bits {
            true => LAST_COMMAND_DONE_A::CPE1,
            false => LAST_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == LAST_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == LAST_COMMAND_DONE_A::CPE0
    }
}
#[doc = "Field `LAST_COMMAND_DONE` writer - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
pub type LAST_COMMAND_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, LAST_COMMAND_DONE_A, O>;
impl<'a, const O: u8> LAST_COMMAND_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONE_A::CPE0)
    }
}
#[doc = "Field `FG_COMMAND_DONE` reader - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
pub type FG_COMMAND_DONE_R = crate::BitReader<FG_COMMAND_DONE_A>;
#[doc = "2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FG_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<FG_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: FG_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl FG_COMMAND_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FG_COMMAND_DONE_A {
        match self.bits {
            true => FG_COMMAND_DONE_A::CPE1,
            false => FG_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == FG_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == FG_COMMAND_DONE_A::CPE0
    }
}
#[doc = "Field `FG_COMMAND_DONE` writer - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
pub type FG_COMMAND_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, FG_COMMAND_DONE_A, O>;
impl<'a, const O: u8> FG_COMMAND_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONE_A::CPE0)
    }
}
#[doc = "Field `LAST_FG_COMMAND_DONE` reader - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
pub type LAST_FG_COMMAND_DONE_R = crate::BitReader<LAST_FG_COMMAND_DONE_A>;
#[doc = "3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAST_FG_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<LAST_FG_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_FG_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl LAST_FG_COMMAND_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_FG_COMMAND_DONE_A {
        match self.bits {
            true => LAST_FG_COMMAND_DONE_A::CPE1,
            false => LAST_FG_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == LAST_FG_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == LAST_FG_COMMAND_DONE_A::CPE0
    }
}
#[doc = "Field `LAST_FG_COMMAND_DONE` writer - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
pub type LAST_FG_COMMAND_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, LAST_FG_COMMAND_DONE_A, O>;
impl<'a, const O: u8> LAST_FG_COMMAND_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONE_A::CPE0)
    }
}
#[doc = "Field `TX_DONE` reader - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
pub type TX_DONE_R = crate::BitReader<TX_DONE_A>;
#[doc = "4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DONE_A {
        match self.bits {
            true => TX_DONE_A::CPE1,
            false => TX_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_DONE_A::CPE0
    }
}
#[doc = "Field `TX_DONE` writer - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
pub type TX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_DONE_A, O>;
impl<'a, const O: u8> TX_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_DONE_A::CPE0)
    }
}
#[doc = "Field `TX_ACK` reader - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
pub type TX_ACK_R = crate::BitReader<TX_ACK_A>;
#[doc = "5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ACK_A {
        match self.bits {
            true => TX_ACK_A::CPE1,
            false => TX_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_ACK_A::CPE0
    }
}
#[doc = "Field `TX_ACK` writer - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
pub type TX_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_ACK_A, O>;
impl<'a, const O: u8> TX_ACK_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ACK_A::CPE0)
    }
}
#[doc = "Field `TX_CTRL` reader - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
pub type TX_CTRL_R = crate::BitReader<TX_CTRL_A>;
#[doc = "6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_CTRL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_A {
        match self.bits {
            true => TX_CTRL_A::CPE1,
            false => TX_CTRL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_A::CPE0
    }
}
#[doc = "Field `TX_CTRL` writer - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
pub type TX_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_CTRL_A, O>;
impl<'a, const O: u8> TX_CTRL_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_A::CPE0)
    }
}
#[doc = "Field `TX_CTRL_ACK` reader - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
pub type TX_CTRL_ACK_R = crate::BitReader<TX_CTRL_ACK_A>;
#[doc = "7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_CTRL_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_CTRL_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_ACK_A {
        match self.bits {
            true => TX_CTRL_ACK_A::CPE1,
            false => TX_CTRL_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_ACK_A::CPE0
    }
}
#[doc = "Field `TX_CTRL_ACK` writer - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
pub type TX_CTRL_ACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_CTRL_ACK_A, O>;
impl<'a, const O: u8> TX_CTRL_ACK_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_A::CPE0)
    }
}
#[doc = "Field `TX_CTRL_ACK_ACK` reader - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
pub type TX_CTRL_ACK_ACK_R = crate::BitReader<TX_CTRL_ACK_ACK_A>;
#[doc = "8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_CTRL_ACK_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_ACK_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_ACK_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_CTRL_ACK_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_ACK_ACK_A {
        match self.bits {
            true => TX_CTRL_ACK_ACK_A::CPE1,
            false => TX_CTRL_ACK_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_ACK_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_ACK_ACK_A::CPE0
    }
}
#[doc = "Field `TX_CTRL_ACK_ACK` writer - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
pub type TX_CTRL_ACK_ACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_CTRL_ACK_ACK_A, O>;
impl<'a, const O: u8> TX_CTRL_ACK_ACK_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACK_A::CPE0)
    }
}
#[doc = "Field `TX_RETRANS` reader - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
pub type TX_RETRANS_R = crate::BitReader<TX_RETRANS_A>;
#[doc = "9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_RETRANS_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_RETRANS_A> for bool {
    #[inline(always)]
    fn from(variant: TX_RETRANS_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_RETRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_RETRANS_A {
        match self.bits {
            true => TX_RETRANS_A::CPE1,
            false => TX_RETRANS_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_RETRANS_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_RETRANS_A::CPE0
    }
}
#[doc = "Field `TX_RETRANS` writer - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
pub type TX_RETRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_RETRANS_A, O>;
impl<'a, const O: u8> TX_RETRANS_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_RETRANS_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_RETRANS_A::CPE0)
    }
}
#[doc = "Field `TX_ENTRY_DONE` reader - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
pub type TX_ENTRY_DONE_R = crate::BitReader<TX_ENTRY_DONE_A>;
#[doc = "10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_ENTRY_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_ENTRY_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ENTRY_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_ENTRY_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ENTRY_DONE_A {
        match self.bits {
            true => TX_ENTRY_DONE_A::CPE1,
            false => TX_ENTRY_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_ENTRY_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_ENTRY_DONE_A::CPE0
    }
}
#[doc = "Field `TX_ENTRY_DONE` writer - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
pub type TX_ENTRY_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_ENTRY_DONE_A, O>;
impl<'a, const O: u8> TX_ENTRY_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONE_A::CPE0)
    }
}
#[doc = "Field `TX_BUFFER_CHANGED` reader - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
pub type TX_BUFFER_CHANGED_R = crate::BitReader<TX_BUFFER_CHANGED_A>;
#[doc = "11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_BUFFER_CHANGED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_BUFFER_CHANGED_A> for bool {
    #[inline(always)]
    fn from(variant: TX_BUFFER_CHANGED_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_BUFFER_CHANGED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_BUFFER_CHANGED_A {
        match self.bits {
            true => TX_BUFFER_CHANGED_A::CPE1,
            false => TX_BUFFER_CHANGED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_BUFFER_CHANGED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_BUFFER_CHANGED_A::CPE0
    }
}
#[doc = "Field `TX_BUFFER_CHANGED` writer - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
pub type TX_BUFFER_CHANGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, TX_BUFFER_CHANGED_A, O>;
impl<'a, const O: u8> TX_BUFFER_CHANGED_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGED_A::CPE0)
    }
}
#[doc = "Field `IRQ12` reader - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
pub type IRQ12_R = crate::BitReader<IRQ12_A>;
#[doc = "12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQ12_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ12_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ12_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQ12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ12_A {
        match self.bits {
            true => IRQ12_A::CPE1,
            false => IRQ12_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ12_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ12_A::CPE0
    }
}
#[doc = "Field `IRQ12` writer - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
pub type IRQ12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, IRQ12_A, O>;
impl<'a, const O: u8> IRQ12_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ12_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ12_A::CPE0)
    }
}
#[doc = "Field `IRQ13` reader - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
pub type IRQ13_R = crate::BitReader<IRQ13_A>;
#[doc = "13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQ13_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ13_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ13_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQ13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ13_A {
        match self.bits {
            true => IRQ13_A::CPE1,
            false => IRQ13_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ13_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ13_A::CPE0
    }
}
#[doc = "Field `IRQ13` writer - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
pub type IRQ13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, IRQ13_A, O>;
impl<'a, const O: u8> IRQ13_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ13_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ13_A::CPE0)
    }
}
#[doc = "Field `IRQ14` reader - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
pub type IRQ14_R = crate::BitReader<IRQ14_A>;
#[doc = "14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQ14_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ14_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ14_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQ14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ14_A {
        match self.bits {
            true => IRQ14_A::CPE1,
            false => IRQ14_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ14_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ14_A::CPE0
    }
}
#[doc = "Field `IRQ14` writer - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
pub type IRQ14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, IRQ14_A, O>;
impl<'a, const O: u8> IRQ14_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ14_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ14_A::CPE0)
    }
}
#[doc = "Field `IRQ15` reader - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
pub type IRQ15_R = crate::BitReader<IRQ15_A>;
#[doc = "15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQ15_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ15_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ15_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQ15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ15_A {
        match self.bits {
            true => IRQ15_A::CPE1,
            false => IRQ15_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ15_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ15_A::CPE0
    }
}
#[doc = "Field `IRQ15` writer - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
pub type IRQ15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, IRQ15_A, O>;
impl<'a, const O: u8> IRQ15_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ15_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ15_A::CPE0)
    }
}
#[doc = "Field `RX_OK` reader - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
pub type RX_OK_R = crate::BitReader<RX_OK_A>;
#[doc = "16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_OK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OK_A {
        match self.bits {
            true => RX_OK_A::CPE1,
            false => RX_OK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_OK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_OK_A::CPE0
    }
}
#[doc = "Field `RX_OK` writer - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
pub type RX_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_OK_A, O>;
impl<'a, const O: u8> RX_OK_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_OK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_OK_A::CPE0)
    }
}
#[doc = "Field `RX_NOK` reader - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
pub type RX_NOK_R = crate::BitReader<RX_NOK_A>;
#[doc = "17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_NOK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_NOK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_NOK_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_NOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_NOK_A {
        match self.bits {
            true => RX_NOK_A::CPE1,
            false => RX_NOK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_NOK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_NOK_A::CPE0
    }
}
#[doc = "Field `RX_NOK` writer - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
pub type RX_NOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_NOK_A, O>;
impl<'a, const O: u8> RX_NOK_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_NOK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_NOK_A::CPE0)
    }
}
#[doc = "Field `RX_IGNORED` reader - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
pub type RX_IGNORED_R = crate::BitReader<RX_IGNORED_A>;
#[doc = "18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_IGNORED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_IGNORED_A> for bool {
    #[inline(always)]
    fn from(variant: RX_IGNORED_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_IGNORED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_IGNORED_A {
        match self.bits {
            true => RX_IGNORED_A::CPE1,
            false => RX_IGNORED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_IGNORED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_IGNORED_A::CPE0
    }
}
#[doc = "Field `RX_IGNORED` writer - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
pub type RX_IGNORED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_IGNORED_A, O>;
impl<'a, const O: u8> RX_IGNORED_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_IGNORED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_IGNORED_A::CPE0)
    }
}
#[doc = "Field `RX_EMPTY` reader - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
pub type RX_EMPTY_R = crate::BitReader<RX_EMPTY_A>;
#[doc = "19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EMPTY_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EMPTY_A {
        match self.bits {
            true => RX_EMPTY_A::CPE1,
            false => RX_EMPTY_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_EMPTY_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_EMPTY_A::CPE0
    }
}
#[doc = "Field `RX_EMPTY` writer - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
pub type RX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_EMPTY_A, O>;
impl<'a, const O: u8> RX_EMPTY_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_EMPTY_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_EMPTY_A::CPE0)
    }
}
#[doc = "Field `RX_CTRL` reader - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
pub type RX_CTRL_R = crate::BitReader<RX_CTRL_A>;
#[doc = "20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_CTRL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CTRL_A {
        match self.bits {
            true => RX_CTRL_A::CPE1,
            false => RX_CTRL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_CTRL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_CTRL_A::CPE0
    }
}
#[doc = "Field `RX_CTRL` writer - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
pub type RX_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_CTRL_A, O>;
impl<'a, const O: u8> RX_CTRL_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRL_A::CPE0)
    }
}
#[doc = "Field `RX_CTRL_ACK` reader - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
pub type RX_CTRL_ACK_R = crate::BitReader<RX_CTRL_ACK_A>;
#[doc = "21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_CTRL_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_CTRL_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CTRL_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_CTRL_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CTRL_ACK_A {
        match self.bits {
            true => RX_CTRL_ACK_A::CPE1,
            false => RX_CTRL_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_CTRL_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_CTRL_ACK_A::CPE0
    }
}
#[doc = "Field `RX_CTRL_ACK` writer - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
pub type RX_CTRL_ACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_CTRL_ACK_A, O>;
impl<'a, const O: u8> RX_CTRL_ACK_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRL_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRL_ACK_A::CPE0)
    }
}
#[doc = "Field `RX_BUF_FULL` reader - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
pub type RX_BUF_FULL_R = crate::BitReader<RX_BUF_FULL_A>;
#[doc = "22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_BUF_FULL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_BUF_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_BUF_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_BUF_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_BUF_FULL_A {
        match self.bits {
            true => RX_BUF_FULL_A::CPE1,
            false => RX_BUF_FULL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_BUF_FULL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_BUF_FULL_A::CPE0
    }
}
#[doc = "Field `RX_BUF_FULL` writer - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
pub type RX_BUF_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_BUF_FULL_A, O>;
impl<'a, const O: u8> RX_BUF_FULL_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_BUF_FULL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_BUF_FULL_A::CPE0)
    }
}
#[doc = "Field `RX_ENTRY_DONE` reader - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
pub type RX_ENTRY_DONE_R = crate::BitReader<RX_ENTRY_DONE_A>;
#[doc = "23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_ENTRY_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_ENTRY_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ENTRY_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_ENTRY_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ENTRY_DONE_A {
        match self.bits {
            true => RX_ENTRY_DONE_A::CPE1,
            false => RX_ENTRY_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_ENTRY_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_ENTRY_DONE_A::CPE0
    }
}
#[doc = "Field `RX_ENTRY_DONE` writer - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
pub type RX_ENTRY_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_ENTRY_DONE_A, O>;
impl<'a, const O: u8> RX_ENTRY_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONE_A::CPE0)
    }
}
#[doc = "Field `RX_DATA_WRITTEN` reader - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
pub type RX_DATA_WRITTEN_R = crate::BitReader<RX_DATA_WRITTEN_A>;
#[doc = "24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_DATA_WRITTEN_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_DATA_WRITTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DATA_WRITTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_DATA_WRITTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DATA_WRITTEN_A {
        match self.bits {
            true => RX_DATA_WRITTEN_A::CPE1,
            false => RX_DATA_WRITTEN_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_DATA_WRITTEN_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_DATA_WRITTEN_A::CPE0
    }
}
#[doc = "Field `RX_DATA_WRITTEN` writer - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
pub type RX_DATA_WRITTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_DATA_WRITTEN_A, O>;
impl<'a, const O: u8> RX_DATA_WRITTEN_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTEN_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTEN_A::CPE0)
    }
}
#[doc = "Field `RX_N_DATA_WRITTEN` reader - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
pub type RX_N_DATA_WRITTEN_R = crate::BitReader<RX_N_DATA_WRITTEN_A>;
#[doc = "25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_N_DATA_WRITTEN_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_N_DATA_WRITTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_N_DATA_WRITTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_N_DATA_WRITTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_N_DATA_WRITTEN_A {
        match self.bits {
            true => RX_N_DATA_WRITTEN_A::CPE1,
            false => RX_N_DATA_WRITTEN_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_N_DATA_WRITTEN_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_N_DATA_WRITTEN_A::CPE0
    }
}
#[doc = "Field `RX_N_DATA_WRITTEN` writer - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
pub type RX_N_DATA_WRITTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_N_DATA_WRITTEN_A, O>;
impl<'a, const O: u8> RX_N_DATA_WRITTEN_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTEN_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTEN_A::CPE0)
    }
}
#[doc = "Field `RX_ABORTED` reader - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
pub type RX_ABORTED_R = crate::BitReader<RX_ABORTED_A>;
#[doc = "26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_ABORTED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_ABORTED_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ABORTED_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_ABORTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ABORTED_A {
        match self.bits {
            true => RX_ABORTED_A::CPE1,
            false => RX_ABORTED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_ABORTED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_ABORTED_A::CPE0
    }
}
#[doc = "Field `RX_ABORTED` writer - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
pub type RX_ABORTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, RX_ABORTED_A, O>;
impl<'a, const O: u8> RX_ABORTED_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ABORTED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ABORTED_A::CPE0)
    }
}
#[doc = "Field `IRQ27` reader - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
pub type IRQ27_R = crate::BitReader<IRQ27_A>;
#[doc = "27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQ27_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ27_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ27_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQ27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ27_A {
        match self.bits {
            true => IRQ27_A::CPE1,
            false => IRQ27_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ27_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ27_A::CPE0
    }
}
#[doc = "Field `IRQ27` writer - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
pub type IRQ27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, IRQ27_A, O>;
impl<'a, const O: u8> IRQ27_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ27_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ27_A::CPE0)
    }
}
#[doc = "Field `SYNTH_NO_LOCK` reader - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
pub type SYNTH_NO_LOCK_R = crate::BitReader<SYNTH_NO_LOCK_A>;
#[doc = "28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNTH_NO_LOCK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<SYNTH_NO_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SYNTH_NO_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNTH_NO_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNTH_NO_LOCK_A {
        match self.bits {
            true => SYNTH_NO_LOCK_A::CPE1,
            false => SYNTH_NO_LOCK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == SYNTH_NO_LOCK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == SYNTH_NO_LOCK_A::CPE0
    }
}
#[doc = "Field `SYNTH_NO_LOCK` writer - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
pub type SYNTH_NO_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, SYNTH_NO_LOCK_A, O>;
impl<'a, const O: u8> SYNTH_NO_LOCK_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCK_A::CPE0)
    }
}
#[doc = "Field `MODULES_UNLOCKED` reader - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
pub type MODULES_UNLOCKED_R = crate::BitReader<MODULES_UNLOCKED_A>;
#[doc = "29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODULES_UNLOCKED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<MODULES_UNLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: MODULES_UNLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
impl MODULES_UNLOCKED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODULES_UNLOCKED_A {
        match self.bits {
            true => MODULES_UNLOCKED_A::CPE1,
            false => MODULES_UNLOCKED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == MODULES_UNLOCKED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == MODULES_UNLOCKED_A::CPE0
    }
}
#[doc = "Field `MODULES_UNLOCKED` writer - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
pub type MODULES_UNLOCKED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, MODULES_UNLOCKED_A, O>;
impl<'a, const O: u8> MODULES_UNLOCKED_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKED_A::CPE0)
    }
}
#[doc = "Field `BOOT_DONE` reader - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
pub type BOOT_DONE_R = crate::BitReader<BOOT_DONE_A>;
#[doc = "30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<BOOT_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOT_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_DONE_A {
        match self.bits {
            true => BOOT_DONE_A::CPE1,
            false => BOOT_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == BOOT_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == BOOT_DONE_A::CPE0
    }
}
#[doc = "Field `BOOT_DONE` writer - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
pub type BOOT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEISL_SPEC, BOOT_DONE_A, O>;
impl<'a, const O: u8> BOOT_DONE_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(BOOT_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(BOOT_DONE_A::CPE0)
    }
}
#[doc = "Field `INTERNAL_ERROR` reader - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
pub type INTERNAL_ERROR_R = crate::BitReader<INTERNAL_ERROR_A>;
#[doc = "31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERNAL_ERROR_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<INTERNAL_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: INTERNAL_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl INTERNAL_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERNAL_ERROR_A {
        match self.bits {
            true => INTERNAL_ERROR_A::CPE1,
            false => INTERNAL_ERROR_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == INTERNAL_ERROR_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == INTERNAL_ERROR_A::CPE0
    }
}
#[doc = "Field `INTERNAL_ERROR` writer - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
pub type INTERNAL_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEISL_SPEC, INTERNAL_ERROR_A, O>;
impl<'a, const O: u8> INTERNAL_ERROR_W<'a, O> {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(INTERNAL_ERROR_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(INTERNAL_ERROR_A::CPE0)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONE_R {
        LAST_COMMAND_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONE_R {
        FG_COMMAND_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONE_R {
        LAST_FG_COMMAND_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TX_ACK_R {
        TX_ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TX_CTRL_R {
        TX_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACK_R {
        TX_CTRL_ACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACK_R {
        TX_CTRL_ACK_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TX_RETRANS_R {
        TX_RETRANS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONE_R {
        TX_ENTRY_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGED_R {
        TX_BUFFER_CHANGED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline(always)]
    pub fn irq12(&self) -> IRQ12_R {
        IRQ12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline(always)]
    pub fn irq13(&self) -> IRQ13_R {
        IRQ13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    pub fn irq14(&self) -> IRQ14_R {
        IRQ14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    pub fn irq15(&self) -> IRQ15_R {
        IRQ15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RX_OK_R {
        RX_OK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RX_NOK_R {
        RX_NOK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RX_IGNORED_R {
        RX_IGNORED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RX_CTRL_R {
        RX_CTRL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACK_R {
        RX_CTRL_ACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RX_BUF_FULL_R {
        RX_BUF_FULL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONE_R {
        RX_ENTRY_DONE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTEN_R {
        RX_DATA_WRITTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTEN_R {
        RX_N_DATA_WRITTEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RX_ABORTED_R {
        RX_ABORTED_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    pub fn irq27(&self) -> IRQ27_R {
        IRQ27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCK_R {
        SYNTH_NO_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKED_R {
        MODULES_UNLOCKED_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    pub fn internal_error(&self) -> INTERNAL_ERROR_R {
        INTERNAL_ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<0> {
        COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn last_command_done(&mut self) -> LAST_COMMAND_DONE_W<1> {
        LAST_COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn fg_command_done(&mut self) -> FG_COMMAND_DONE_W<2> {
        FG_COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn last_fg_command_done(&mut self) -> LAST_FG_COMMAND_DONE_W<3> {
        LAST_FG_COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TX_DONE_W<4> {
        TX_DONE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack(&mut self) -> TX_ACK_W<5> {
        TX_ACK_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl(&mut self) -> TX_CTRL_W<6> {
        TX_CTRL_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack(&mut self) -> TX_CTRL_ACK_W<7> {
        TX_CTRL_ACK_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack_ack(&mut self) -> TX_CTRL_ACK_ACK_W<8> {
        TX_CTRL_ACK_ACK_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_retrans(&mut self) -> TX_RETRANS_W<9> {
        TX_RETRANS_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_entry_done(&mut self) -> TX_ENTRY_DONE_W<10> {
        TX_ENTRY_DONE_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_buffer_changed(&mut self) -> TX_BUFFER_CHANGED_W<11> {
        TX_BUFFER_CHANGED_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq12(&mut self) -> IRQ12_W<12> {
        IRQ12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq13(&mut self) -> IRQ13_W<13> {
        IRQ13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq14(&mut self) -> IRQ14_W<14> {
        IRQ14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq15(&mut self) -> IRQ15_W<15> {
        IRQ15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ok(&mut self) -> RX_OK_W<16> {
        RX_OK_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_nok(&mut self) -> RX_NOK_W<17> {
        RX_NOK_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ignored(&mut self) -> RX_IGNORED_W<18> {
        RX_IGNORED_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W<19> {
        RX_EMPTY_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl(&mut self) -> RX_CTRL_W<20> {
        RX_CTRL_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl_ack(&mut self) -> RX_CTRL_ACK_W<21> {
        RX_CTRL_ACK_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_full(&mut self) -> RX_BUF_FULL_W<22> {
        RX_BUF_FULL_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_entry_done(&mut self) -> RX_ENTRY_DONE_W<23> {
        RX_ENTRY_DONE_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_written(&mut self) -> RX_DATA_WRITTEN_W<24> {
        RX_DATA_WRITTEN_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_n_data_written(&mut self) -> RX_N_DATA_WRITTEN_W<25> {
        RX_N_DATA_WRITTEN_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_aborted(&mut self) -> RX_ABORTED_W<26> {
        RX_ABORTED_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq27(&mut self) -> IRQ27_W<27> {
        IRQ27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn synth_no_lock(&mut self) -> SYNTH_NO_LOCK_W<28> {
        SYNTH_NO_LOCK_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn modules_unlocked(&mut self) -> MODULES_UNLOCKED_W<29> {
        MODULES_UNLOCKED_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn boot_done(&mut self) -> BOOT_DONE_W<30> {
        BOOT_DONE_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn internal_error(&mut self) -> INTERNAL_ERROR_W<31> {
        INTERNAL_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcpeisl](index.html) module"]
pub struct RFCPEISL_SPEC;
impl crate::RegisterSpec for RFCPEISL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcpeisl::R](R) reader structure"]
impl crate::Readable for RFCPEISL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcpeisl::W](W) writer structure"]
impl crate::Writable for RFCPEISL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCPEISL to value 0xffff_0000"]
impl crate::Resettable for RFCPEISL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
