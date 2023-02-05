#[doc = "Register `STATADDR` reader"]
pub struct R(crate::R<STATADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATADDR` writer"]
pub struct W(crate::W<STATADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATADDR_SPEC>;
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
impl From<crate::W<STATADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANKADDR` reader - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
pub type BANKADDR_R = crate::FieldReader<u16, BANKADDR_A>;
#[doc = "15:0\\]
Current Bank Address A bank offset address is stored in this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BANKADDR_A {
    #[doc = "65535: Maximum value"]
    MAXIMUM = 65535,
    #[doc = "0: Minimum value"]
    MINIMUM = 0,
}
impl From<BANKADDR_A> for u16 {
    #[inline(always)]
    fn from(variant: BANKADDR_A) -> Self {
        variant as _
    }
}
impl BANKADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BANKADDR_A> {
        match self.bits {
            65535 => Some(BANKADDR_A::MAXIMUM),
            0 => Some(BANKADDR_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == BANKADDR_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == BANKADDR_A::MINIMUM
    }
}
#[doc = "Field `BANKADDR` writer - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
pub type BANKADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATADDR_SPEC, u16, BANKADDR_A, 16, O>;
impl<'a, const O: u8> BANKADDR_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(BANKADDR_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(BANKADDR_A::MINIMUM)
    }
}
#[doc = "Field `REGIONID` reader - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
pub type REGIONID_R = crate::FieldReader<u8, REGIONID_A>;
#[doc = "20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REGIONID_A {
    #[doc = "8: Engr Region"]
    ENGR = 8,
    #[doc = "4: Trim Region"]
    TRIM = 4,
    #[doc = "2: Non-Main Region"]
    NONMAIN = 2,
    #[doc = "1: Main Region"]
    MAIN = 1,
}
impl From<REGIONID_A> for u8 {
    #[inline(always)]
    fn from(variant: REGIONID_A) -> Self {
        variant as _
    }
}
impl REGIONID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REGIONID_A> {
        match self.bits {
            8 => Some(REGIONID_A::ENGR),
            4 => Some(REGIONID_A::TRIM),
            2 => Some(REGIONID_A::NONMAIN),
            1 => Some(REGIONID_A::MAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENGR`"]
    #[inline(always)]
    pub fn is_engr(&self) -> bool {
        *self == REGIONID_A::ENGR
    }
    #[doc = "Checks if the value of the field is `TRIM`"]
    #[inline(always)]
    pub fn is_trim(&self) -> bool {
        *self == REGIONID_A::TRIM
    }
    #[doc = "Checks if the value of the field is `NONMAIN`"]
    #[inline(always)]
    pub fn is_nonmain(&self) -> bool {
        *self == REGIONID_A::NONMAIN
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == REGIONID_A::MAIN
    }
}
#[doc = "Field `REGIONID` writer - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
pub type REGIONID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATADDR_SPEC, u8, REGIONID_A, 5, O>;
impl<'a, const O: u8> REGIONID_W<'a, O> {
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn engr(self) -> &'a mut W {
        self.variant(REGIONID_A::ENGR)
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn trim(self) -> &'a mut W {
        self.variant(REGIONID_A::TRIM)
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn nonmain(self) -> &'a mut W {
        self.variant(REGIONID_A::NONMAIN)
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(REGIONID_A::MAIN)
    }
}
#[doc = "Field `BANKID` reader - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
pub type BANKID_R = crate::FieldReader<u8, BANKID_A>;
#[doc = "25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKID_A {
    #[doc = "16: Bank 4"]
    BANK4 = 16,
    #[doc = "8: Bank 3"]
    BANK3 = 8,
    #[doc = "4: Bank 2"]
    BANK2 = 4,
    #[doc = "2: Bank 1"]
    BANK1 = 2,
    #[doc = "1: Bank 0"]
    BANK0 = 1,
}
impl From<BANKID_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKID_A) -> Self {
        variant as _
    }
}
impl BANKID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BANKID_A> {
        match self.bits {
            16 => Some(BANKID_A::BANK4),
            8 => Some(BANKID_A::BANK3),
            4 => Some(BANKID_A::BANK2),
            2 => Some(BANKID_A::BANK1),
            1 => Some(BANKID_A::BANK0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK4`"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == BANKID_A::BANK4
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKID_A::BANK3
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKID_A::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKID_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKID_A::BANK0
    }
}
#[doc = "Field `BANKID` writer - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
pub type BANKID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATADDR_SPEC, u8, BANKID_A, 5, O>;
impl<'a, const O: u8> BANKID_W<'a, O> {
    #[doc = "Bank 4"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut W {
        self.variant(BANKID_A::BANK4)
    }
    #[doc = "Bank 3"]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKID_A::BANK3)
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKID_A::BANK2)
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKID_A::BANK1)
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKID_A::BANK0)
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATADDR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
    #[inline(always)]
    pub fn bankaddr(&self) -> BANKADDR_R {
        BANKADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
    #[inline(always)]
    pub fn regionid(&self) -> REGIONID_R {
        REGIONID_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
    #[inline(always)]
    pub fn bankid(&self) -> BANKID_R {
        BANKID_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
    #[inline(always)]
    #[must_use]
    pub fn bankaddr(&mut self) -> BANKADDR_W<0> {
        BANKADDR_W::new(self)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
    #[inline(always)]
    #[must_use]
    pub fn regionid(&mut self) -> REGIONID_W<16> {
        REGIONID_W::new(self)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
    #[inline(always)]
    #[must_use]
    pub fn bankid(&mut self) -> BANKID_W<21> {
        BANKID_W::new(self)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> RESERVED26_W<26> {
        RESERVED26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Address Counter Value Read only register giving read access to the state machine current address. A bank id, region id and address are stored in this register and are incremented as necessary during execution of a command.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stataddr](index.html) module"]
pub struct STATADDR_SPEC;
impl crate::RegisterSpec for STATADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stataddr::R](R) reader structure"]
impl crate::Readable for STATADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stataddr::W](W) writer structure"]
impl crate::Writable for STATADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATADDR to value 0x0020_0000"]
impl crate::Resettable for STATADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
