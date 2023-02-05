#[doc = "Register `SHCSR` reader"]
pub struct R(crate::R<SHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHCSR` writer"]
pub struct W(crate::W<SHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHCSR_SPEC>;
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
impl From<crate::W<SHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMFAULTACT` reader - 0:0\\]
MemManage exception active"]
pub type MEMFAULTACT_R = crate::BitReader<MEMFAULTACT_A>;
#[doc = "0:0\\]
MemManage exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<MEMFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTACT_A {
        match self.bits {
            true => MEMFAULTACT_A::ACTIVE,
            false => MEMFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MEMFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == MEMFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Field `MEMFAULTACT` writer - 0:0\\]
MemManage exception active"]
pub type MEMFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, MEMFAULTACT_A, O>;
impl<'a, const O: u8> MEMFAULTACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::NOTACTIVE)
    }
}
#[doc = "Field `BUSFAULTACT` reader - 1:1\\]
BusFault exception active"]
pub type BUSFAULTACT_R = crate::BitReader<BUSFAULTACT_A>;
#[doc = "1:1\\]
BusFault exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<BUSFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTACT_A {
        match self.bits {
            true => BUSFAULTACT_A::ACTIVE,
            false => BUSFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == BUSFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Field `BUSFAULTACT` writer - 1:1\\]
BusFault exception active"]
pub type BUSFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, BUSFAULTACT_A, O>;
impl<'a, const O: u8> BUSFAULTACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::NOTACTIVE)
    }
}
#[doc = "Field `HARDFAULTACT` reader - 2:2\\]
HardFault exception active state. Indicates and allows limited modification of the active state of the HardFault exception for the selected Security state"]
pub type HARDFAULTACT_R = crate::BitReader<HARDFAULTACT_A>;
#[doc = "2:2\\]
HardFault exception active state. Indicates and allows limited modification of the active state of the HardFault exception for the selected Security state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HARDFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<HARDFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: HARDFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl HARDFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HARDFAULTACT_A {
        match self.bits {
            true => HARDFAULTACT_A::ACTIVE,
            false => HARDFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HARDFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == HARDFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Field `HARDFAULTACT` writer - 2:2\\]
HardFault exception active state. Indicates and allows limited modification of the active state of the HardFault exception for the selected Security state"]
pub type HARDFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, HARDFAULTACT_A, O>;
impl<'a, const O: u8> HARDFAULTACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(HARDFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(HARDFAULTACT_A::NOTACTIVE)
    }
}
#[doc = "Field `USGFAULTACT` reader - 3:3\\]
UsageFault exception active"]
pub type USGFAULTACT_R = crate::BitReader<USGFAULTACT_A>;
#[doc = "3:3\\]
UsageFault exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<USGFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl USGFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTACT_A {
        match self.bits {
            true => USGFAULTACT_A::ACTIVE,
            false => USGFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == USGFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == USGFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Field `USGFAULTACT` writer - 3:3\\]
UsageFault exception active"]
pub type USGFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, USGFAULTACT_A, O>;
impl<'a, const O: u8> USGFAULTACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::NOTACTIVE)
    }
}
#[doc = "Field `SECUREFAULTACT` reader - 4:4\\]
SecureFault exception active state"]
pub type SECUREFAULTACT_R = crate::BitReader<SECUREFAULTACT_A>;
#[doc = "4:4\\]
SecureFault exception active state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECUREFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<SECUREFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: SECUREFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl SECUREFAULTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECUREFAULTACT_A {
        match self.bits {
            true => SECUREFAULTACT_A::ACTIVE,
            false => SECUREFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SECUREFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == SECUREFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Field `SECUREFAULTACT` writer - 4:4\\]
SecureFault exception active state"]
pub type SECUREFAULTACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, SECUREFAULTACT_A, O>;
impl<'a, const O: u8> SECUREFAULTACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SECUREFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(SECUREFAULTACT_A::NOTACTIVE)
    }
}
#[doc = "Field `NMIACT` reader - 5:5\\]
NMI exception active state"]
pub type NMIACT_R = crate::BitReader<NMIACT_A>;
#[doc = "5:5\\]
NMI exception active state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<NMIACT_A> for bool {
    #[inline(always)]
    fn from(variant: NMIACT_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIACT_A {
        match self.bits {
            true => NMIACT_A::ACTIVE,
            false => NMIACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == NMIACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == NMIACT_A::NOTACTIVE
    }
}
#[doc = "Field `NMIACT` writer - 5:5\\]
NMI exception active state"]
pub type NMIACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, NMIACT_A, O>;
impl<'a, const O: u8> NMIACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(NMIACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(NMIACT_A::NOTACTIVE)
    }
}
#[doc = "Field `RESERVED6` reader - 6:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED6` writer - 6:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `SVCALLACT` reader - 7:7\\]
SVCall active"]
pub type SVCALLACT_R = crate::BitReader<SVCALLACT_A>;
#[doc = "7:7\\]
SVCall active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCALLACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<SVCALLACT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLACT_A) -> Self {
        variant as u8 != 0
    }
}
impl SVCALLACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLACT_A {
        match self.bits {
            true => SVCALLACT_A::ACTIVE,
            false => SVCALLACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SVCALLACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == SVCALLACT_A::NOTACTIVE
    }
}
#[doc = "Field `SVCALLACT` writer - 7:7\\]
SVCall active"]
pub type SVCALLACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, SVCALLACT_A, O>;
impl<'a, const O: u8> SVCALLACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SVCALLACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(SVCALLACT_A::NOTACTIVE)
    }
}
#[doc = "Field `MONITORACT` reader - 8:8\\]
Debug monitor active"]
pub type MONITORACT_R = crate::BitReader<MONITORACT_A>;
#[doc = "8:8\\]
Debug monitor active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONITORACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<MONITORACT_A> for bool {
    #[inline(always)]
    fn from(variant: MONITORACT_A) -> Self {
        variant as u8 != 0
    }
}
impl MONITORACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONITORACT_A {
        match self.bits {
            true => MONITORACT_A::ACTIVE,
            false => MONITORACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MONITORACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == MONITORACT_A::NOTACTIVE
    }
}
#[doc = "Field `MONITORACT` writer - 8:8\\]
Debug monitor active"]
pub type MONITORACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, MONITORACT_A, O>;
impl<'a, const O: u8> MONITORACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(MONITORACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(MONITORACT_A::NOTACTIVE)
    }
}
#[doc = "Field `RESERVED9` reader - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED9` writer - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `PENDSVACT` reader - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
pub type PENDSVACT_R = crate::BitReader<bool>;
#[doc = "Field `PENDSVACT` writer - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
pub type PENDSVACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `SYSTICKACT` reader - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
pub type SYSTICKACT_R = crate::BitReader<SYSTICKACT_A>;
#[doc = "11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSTICKACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<SYSTICKACT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSTICKACT_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSTICKACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTICKACT_A {
        match self.bits {
            true => SYSTICKACT_A::ACTIVE,
            false => SYSTICKACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSTICKACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == SYSTICKACT_A::NOTACTIVE
    }
}
#[doc = "Field `SYSTICKACT` writer - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
pub type SYSTICKACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, SYSTICKACT_A, O>;
impl<'a, const O: u8> SYSTICKACT_W<'a, O> {
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::NOTACTIVE)
    }
}
#[doc = "Field `USGFAULTPENDED` reader - 12:12\\]
Usage fault pending"]
pub type USGFAULTPENDED_R = crate::BitReader<USGFAULTPENDED_A>;
#[doc = "12:12\\]
Usage fault pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<USGFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl USGFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTPENDED_A {
        match self.bits {
            true => USGFAULTPENDED_A::PENDING,
            false => USGFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == USGFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == USGFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Field `USGFAULTPENDED` writer - 12:12\\]
Usage fault pending"]
pub type USGFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, USGFAULTPENDED_A, O>;
impl<'a, const O: u8> USGFAULTPENDED_W<'a, O> {
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::NOTPENDING)
    }
}
#[doc = "Field `MEMFAULTPENDED` reader - 13:13\\]
MemManage exception pending"]
pub type MEMFAULTPENDED_R = crate::BitReader<MEMFAULTPENDED_A>;
#[doc = "13:13\\]
MemManage exception pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<MEMFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTPENDED_A {
        match self.bits {
            true => MEMFAULTPENDED_A::PENDING,
            false => MEMFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MEMFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == MEMFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Field `MEMFAULTPENDED` writer - 13:13\\]
MemManage exception pending"]
pub type MEMFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, MEMFAULTPENDED_A, O>;
impl<'a, const O: u8> MEMFAULTPENDED_W<'a, O> {
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::NOTPENDING)
    }
}
#[doc = "Field `BUSFAULTPENDED` reader - 14:14\\]
BusFault pending"]
pub type BUSFAULTPENDED_R = crate::BitReader<BUSFAULTPENDED_A>;
#[doc = "14:14\\]
BusFault pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<BUSFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTPENDED_A {
        match self.bits {
            true => BUSFAULTPENDED_A::PENDING,
            false => BUSFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BUSFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == BUSFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Field `BUSFAULTPENDED` writer - 14:14\\]
BusFault pending"]
pub type BUSFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, BUSFAULTPENDED_A, O>;
impl<'a, const O: u8> BUSFAULTPENDED_W<'a, O> {
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::NOTPENDING)
    }
}
#[doc = "Field `SVCALLPENDED` reader - 15:15\\]
SVCall pending"]
pub type SVCALLPENDED_R = crate::BitReader<SVCALLPENDED_A>;
#[doc = "15:15\\]
SVCall pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCALLPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<SVCALLPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl SVCALLPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLPENDED_A {
        match self.bits {
            true => SVCALLPENDED_A::PENDING,
            false => SVCALLPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SVCALLPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == SVCALLPENDED_A::NOTPENDING
    }
}
#[doc = "Field `SVCALLPENDED` writer - 15:15\\]
SVCall pending"]
pub type SVCALLPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, SVCALLPENDED_A, O>;
impl<'a, const O: u8> SVCALLPENDED_W<'a, O> {
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::NOTPENDING)
    }
}
#[doc = "Field `MEMFAULTENA` reader - 16:16\\]
MemManage fault system handler enable"]
pub type MEMFAULTENA_R = crate::BitReader<MEMFAULTENA_A>;
#[doc = "16:16\\]
MemManage fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMFAULTENA_A {
    #[doc = "1: Exception enabled"]
    EN = 1,
    #[doc = "0: Exception disabled"]
    DIS = 0,
}
impl From<MEMFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMFAULTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTENA_A {
        match self.bits {
            true => MEMFAULTENA_A::EN,
            false => MEMFAULTENA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEMFAULTENA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MEMFAULTENA_A::DIS
    }
}
#[doc = "Field `MEMFAULTENA` writer - 16:16\\]
MemManage fault system handler enable"]
pub type MEMFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, MEMFAULTENA_A, O>;
impl<'a, const O: u8> MEMFAULTENA_W<'a, O> {
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::EN)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::DIS)
    }
}
#[doc = "Field `BUSFAULTENA` reader - 17:17\\]
Bus fault system handler enable"]
pub type BUSFAULTENA_R = crate::BitReader<BUSFAULTENA_A>;
#[doc = "17:17\\]
Bus fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSFAULTENA_A {
    #[doc = "1: Exception enabled"]
    EN = 1,
    #[doc = "0: Exception disabled"]
    DIS = 0,
}
impl From<BUSFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSFAULTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTENA_A {
        match self.bits {
            true => BUSFAULTENA_A::EN,
            false => BUSFAULTENA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BUSFAULTENA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BUSFAULTENA_A::DIS
    }
}
#[doc = "Field `BUSFAULTENA` writer - 17:17\\]
Bus fault system handler enable"]
pub type BUSFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, BUSFAULTENA_A, O>;
impl<'a, const O: u8> BUSFAULTENA_W<'a, O> {
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::EN)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::DIS)
    }
}
#[doc = "Field `USGFAULTENA` reader - 18:18\\]
Usage fault system handler enable"]
pub type USGFAULTENA_R = crate::BitReader<USGFAULTENA_A>;
#[doc = "18:18\\]
Usage fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGFAULTENA_A {
    #[doc = "1: Exception enabled"]
    EN = 1,
    #[doc = "0: Exception disabled"]
    DIS = 0,
}
impl From<USGFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl USGFAULTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTENA_A {
        match self.bits {
            true => USGFAULTENA_A::EN,
            false => USGFAULTENA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == USGFAULTENA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == USGFAULTENA_A::DIS
    }
}
#[doc = "Field `USGFAULTENA` writer - 18:18\\]
Usage fault system handler enable"]
pub type USGFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, USGFAULTENA_A, O>;
impl<'a, const O: u8> USGFAULTENA_W<'a, O> {
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::EN)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::DIS)
    }
}
#[doc = "Field `SECUREFAULTENA` reader - 19:19\\]
SecureFault exception enable."]
pub type SECUREFAULTENA_R = crate::BitReader<SECUREFAULTENA_A>;
#[doc = "19:19\\]
SecureFault exception enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECUREFAULTENA_A {
    #[doc = "1: Exception enabled"]
    EN = 1,
    #[doc = "0: Exception disabled"]
    DIS = 0,
}
impl From<SECUREFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: SECUREFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl SECUREFAULTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECUREFAULTENA_A {
        match self.bits {
            true => SECUREFAULTENA_A::EN,
            false => SECUREFAULTENA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SECUREFAULTENA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SECUREFAULTENA_A::DIS
    }
}
#[doc = "Field `SECUREFAULTENA` writer - 19:19\\]
SecureFault exception enable."]
pub type SECUREFAULTENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, SECUREFAULTENA_A, O>;
impl<'a, const O: u8> SECUREFAULTENA_W<'a, O> {
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SECUREFAULTENA_A::EN)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SECUREFAULTENA_A::DIS)
    }
}
#[doc = "Field `SECUREFAULTPENDED` reader - 20:20\\]
SecureFault exception pended state"]
pub type SECUREFAULTPENDED_R = crate::BitReader<SECUREFAULTPENDED_A>;
#[doc = "20:20\\]
SecureFault exception pended state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECUREFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<SECUREFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SECUREFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl SECUREFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECUREFAULTPENDED_A {
        match self.bits {
            true => SECUREFAULTPENDED_A::PENDING,
            false => SECUREFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SECUREFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == SECUREFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Field `SECUREFAULTPENDED` writer - 20:20\\]
SecureFault exception pended state"]
pub type SECUREFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, SECUREFAULTPENDED_A, O>;
impl<'a, const O: u8> SECUREFAULTPENDED_W<'a, O> {
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SECUREFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(SECUREFAULTPENDED_A::NOTPENDING)
    }
}
#[doc = "Field `HARDFAULTPENDED` reader - 21:21\\]
SecureFault exception pended state"]
pub type HARDFAULTPENDED_R = crate::BitReader<HARDFAULTPENDED_A>;
#[doc = "21:21\\]
SecureFault exception pended state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HARDFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<HARDFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: HARDFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl HARDFAULTPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HARDFAULTPENDED_A {
        match self.bits {
            true => HARDFAULTPENDED_A::PENDING,
            false => HARDFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == HARDFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == HARDFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Field `HARDFAULTPENDED` writer - 21:21\\]
SecureFault exception pended state"]
pub type HARDFAULTPENDED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHCSR_SPEC, HARDFAULTPENDED_A, O>;
impl<'a, const O: u8> HARDFAULTPENDED_W<'a, O> {
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(HARDFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(HARDFAULTPENDED_A::NOTPENDING)
    }
}
#[doc = "Field `RESERVED22` reader - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED22` writer - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHCSR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MemManage exception active"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BusFault exception active"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
HardFault exception active state. Indicates and allows limited modification of the active state of the HardFault exception for the selected Security state"]
    #[inline(always)]
    pub fn hardfaultact(&self) -> HARDFAULTACT_R {
        HARDFAULTACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
UsageFault exception active"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SecureFault exception active state"]
    #[inline(always)]
    pub fn securefaultact(&self) -> SECUREFAULTACT_R {
        SECUREFAULTACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
NMI exception active state"]
    #[inline(always)]
    pub fn nmiact(&self) -> NMIACT_R {
        NMIACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SVCall active"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug monitor active"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Usage fault pending"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
MemManage exception pending"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
BusFault pending"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
SVCall pending"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MemManage fault system handler enable"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Bus fault system handler enable"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Usage fault system handler enable"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
SecureFault exception enable."]
    #[inline(always)]
    pub fn securefaultena(&self) -> SECUREFAULTENA_R {
        SECUREFAULTENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
SecureFault exception pended state"]
    #[inline(always)]
    pub fn securefaultpended(&self) -> SECUREFAULTPENDED_R {
        SECUREFAULTPENDED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
SecureFault exception pended state"]
    #[inline(always)]
    pub fn hardfaultpended(&self) -> HARDFAULTPENDED_R {
        HARDFAULTPENDED_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MemManage exception active"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W<0> {
        MEMFAULTACT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
BusFault exception active"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W<1> {
        BUSFAULTACT_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
HardFault exception active state. Indicates and allows limited modification of the active state of the HardFault exception for the selected Security state"]
    #[inline(always)]
    #[must_use]
    pub fn hardfaultact(&mut self) -> HARDFAULTACT_W<2> {
        HARDFAULTACT_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
UsageFault exception active"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W<3> {
        USGFAULTACT_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
SecureFault exception active state"]
    #[inline(always)]
    #[must_use]
    pub fn securefaultact(&mut self) -> SECUREFAULTACT_W<4> {
        SECUREFAULTACT_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
NMI exception active state"]
    #[inline(always)]
    #[must_use]
    pub fn nmiact(&mut self) -> NMIACT_W<5> {
        NMIACT_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
SVCall active"]
    #[inline(always)]
    #[must_use]
    pub fn svcallact(&mut self) -> SVCALLACT_W<7> {
        SVCALLACT_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug monitor active"]
    #[inline(always)]
    #[must_use]
    pub fn monitoract(&mut self) -> MONITORACT_W<8> {
        MONITORACT_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvact(&mut self) -> PENDSVACT_W<10> {
        PENDSVACT_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
    #[inline(always)]
    #[must_use]
    pub fn systickact(&mut self) -> SYSTICKACT_W<11> {
        SYSTICKACT_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Usage fault pending"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W<12> {
        USGFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
MemManage exception pending"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W<13> {
        MEMFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
BusFault pending"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W<14> {
        BUSFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
SVCall pending"]
    #[inline(always)]
    #[must_use]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W<15> {
        SVCALLPENDED_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
MemManage fault system handler enable"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W<16> {
        MEMFAULTENA_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Bus fault system handler enable"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W<17> {
        BUSFAULTENA_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Usage fault system handler enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W<18> {
        USGFAULTENA_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
SecureFault exception enable."]
    #[inline(always)]
    #[must_use]
    pub fn securefaultena(&mut self) -> SECUREFAULTENA_W<19> {
        SECUREFAULTENA_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
SecureFault exception pended state"]
    #[inline(always)]
    #[must_use]
    pub fn securefaultpended(&mut self) -> SECUREFAULTPENDED_W<20> {
        SECUREFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
SecureFault exception pended state"]
    #[inline(always)]
    #[must_use]
    pub fn hardfaultpended(&mut self) -> HARDFAULTPENDED_W<21> {
        HARDFAULTPENDED_W::new(self)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> RESERVED22_W<22> {
        RESERVED22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](index.html) module"]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shcsr::R](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shcsr::W](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
