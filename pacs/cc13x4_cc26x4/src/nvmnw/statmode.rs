#[doc = "Register `STATMODE` reader"]
pub struct R(crate::R<STATMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATMODE` writer"]
pub struct W(crate::W<STATMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATMODE_SPEC>;
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
impl From<crate::W<STATMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANKNOTINRD` reader - 4:0\\]
Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank."]
pub type BANKNOTINRD_R = crate::FieldReader<u8, BANKNOTINRD_A>;
#[doc = "4:0\\]
Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKNOTINRD_A {
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
impl From<BANKNOTINRD_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKNOTINRD_A) -> Self {
        variant as _
    }
}
impl BANKNOTINRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BANKNOTINRD_A> {
        match self.bits {
            16 => Some(BANKNOTINRD_A::BANK4),
            8 => Some(BANKNOTINRD_A::BANK3),
            4 => Some(BANKNOTINRD_A::BANK2),
            2 => Some(BANKNOTINRD_A::BANK1),
            1 => Some(BANKNOTINRD_A::BANK0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK4`"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == BANKNOTINRD_A::BANK4
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKNOTINRD_A::BANK3
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKNOTINRD_A::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKNOTINRD_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKNOTINRD_A::BANK0
    }
}
#[doc = "Field `BANKNOTINRD` writer - 4:0\\]
Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank."]
pub type BANKNOTINRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATMODE_SPEC, u8, BANKNOTINRD_A, 5, O>;
impl<'a, const O: u8> BANKNOTINRD_W<'a, O> {
    #[doc = "Bank 4"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut W {
        self.variant(BANKNOTINRD_A::BANK4)
    }
    #[doc = "Bank 3"]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKNOTINRD_A::BANK3)
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKNOTINRD_A::BANK2)
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKNOTINRD_A::BANK1)
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKNOTINRD_A::BANK0)
    }
}
#[doc = "Field `RESERVED1` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATMODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `BANKMODE` reader - 11:8\\]
Indicates mode of bank(s) that are not in READ mode"]
pub type BANKMODE_R = crate::FieldReader<u8, BANKMODE_A>;
#[doc = "11:8\\]
Indicates mode of bank(s) that are not in READ mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKMODE_A {
    #[doc = "15: Erase Bank"]
    ERASEBNK = 15,
    #[doc = "14: Program Multiple Word"]
    PGMMW = 14,
    #[doc = "12: Erase Sector"]
    ERASESECT = 12,
    #[doc = "11: Erase Verify Mode"]
    ERASEVER = 11,
    #[doc = "10: Program Single Word"]
    PGMSW = 10,
    #[doc = "9: Program Verify Mode"]
    PGMVER = 9,
    #[doc = "7: Read Margin 1B Mode"]
    RDMARG1B = 7,
    #[doc = "6: Read Margin 0B Mode"]
    RDMARG0B = 6,
    #[doc = "4: Read Margin 1 Mode"]
    RDMARG1 = 4,
    #[doc = "2: Read Margin 0 Mode"]
    RDMARG0 = 2,
    #[doc = "0: Read Mode"]
    READ = 0,
}
impl From<BANKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKMODE_A) -> Self {
        variant as _
    }
}
impl BANKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BANKMODE_A> {
        match self.bits {
            15 => Some(BANKMODE_A::ERASEBNK),
            14 => Some(BANKMODE_A::PGMMW),
            12 => Some(BANKMODE_A::ERASESECT),
            11 => Some(BANKMODE_A::ERASEVER),
            10 => Some(BANKMODE_A::PGMSW),
            9 => Some(BANKMODE_A::PGMVER),
            7 => Some(BANKMODE_A::RDMARG1B),
            6 => Some(BANKMODE_A::RDMARG0B),
            4 => Some(BANKMODE_A::RDMARG1),
            2 => Some(BANKMODE_A::RDMARG0),
            0 => Some(BANKMODE_A::READ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ERASEBNK`"]
    #[inline(always)]
    pub fn is_erasebnk(&self) -> bool {
        *self == BANKMODE_A::ERASEBNK
    }
    #[doc = "Checks if the value of the field is `PGMMW`"]
    #[inline(always)]
    pub fn is_pgmmw(&self) -> bool {
        *self == BANKMODE_A::PGMMW
    }
    #[doc = "Checks if the value of the field is `ERASESECT`"]
    #[inline(always)]
    pub fn is_erasesect(&self) -> bool {
        *self == BANKMODE_A::ERASESECT
    }
    #[doc = "Checks if the value of the field is `ERASEVER`"]
    #[inline(always)]
    pub fn is_erasever(&self) -> bool {
        *self == BANKMODE_A::ERASEVER
    }
    #[doc = "Checks if the value of the field is `PGMSW`"]
    #[inline(always)]
    pub fn is_pgmsw(&self) -> bool {
        *self == BANKMODE_A::PGMSW
    }
    #[doc = "Checks if the value of the field is `PGMVER`"]
    #[inline(always)]
    pub fn is_pgmver(&self) -> bool {
        *self == BANKMODE_A::PGMVER
    }
    #[doc = "Checks if the value of the field is `RDMARG1B`"]
    #[inline(always)]
    pub fn is_rdmarg1b(&self) -> bool {
        *self == BANKMODE_A::RDMARG1B
    }
    #[doc = "Checks if the value of the field is `RDMARG0B`"]
    #[inline(always)]
    pub fn is_rdmarg0b(&self) -> bool {
        *self == BANKMODE_A::RDMARG0B
    }
    #[doc = "Checks if the value of the field is `RDMARG1`"]
    #[inline(always)]
    pub fn is_rdmarg1(&self) -> bool {
        *self == BANKMODE_A::RDMARG1
    }
    #[doc = "Checks if the value of the field is `RDMARG0`"]
    #[inline(always)]
    pub fn is_rdmarg0(&self) -> bool {
        *self == BANKMODE_A::RDMARG0
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == BANKMODE_A::READ
    }
}
#[doc = "Field `BANKMODE` writer - 11:8\\]
Indicates mode of bank(s) that are not in READ mode"]
pub type BANKMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATMODE_SPEC, u8, BANKMODE_A, 4, O>;
impl<'a, const O: u8> BANKMODE_W<'a, O> {
    #[doc = "Erase Bank"]
    #[inline(always)]
    pub fn erasebnk(self) -> &'a mut W {
        self.variant(BANKMODE_A::ERASEBNK)
    }
    #[doc = "Program Multiple Word"]
    #[inline(always)]
    pub fn pgmmw(self) -> &'a mut W {
        self.variant(BANKMODE_A::PGMMW)
    }
    #[doc = "Erase Sector"]
    #[inline(always)]
    pub fn erasesect(self) -> &'a mut W {
        self.variant(BANKMODE_A::ERASESECT)
    }
    #[doc = "Erase Verify Mode"]
    #[inline(always)]
    pub fn erasever(self) -> &'a mut W {
        self.variant(BANKMODE_A::ERASEVER)
    }
    #[doc = "Program Single Word"]
    #[inline(always)]
    pub fn pgmsw(self) -> &'a mut W {
        self.variant(BANKMODE_A::PGMSW)
    }
    #[doc = "Program Verify Mode"]
    #[inline(always)]
    pub fn pgmver(self) -> &'a mut W {
        self.variant(BANKMODE_A::PGMVER)
    }
    #[doc = "Read Margin 1B Mode"]
    #[inline(always)]
    pub fn rdmarg1b(self) -> &'a mut W {
        self.variant(BANKMODE_A::RDMARG1B)
    }
    #[doc = "Read Margin 0B Mode"]
    #[inline(always)]
    pub fn rdmarg0b(self) -> &'a mut W {
        self.variant(BANKMODE_A::RDMARG0B)
    }
    #[doc = "Read Margin 1 Mode"]
    #[inline(always)]
    pub fn rdmarg1(self) -> &'a mut W {
        self.variant(BANKMODE_A::RDMARG1)
    }
    #[doc = "Read Margin 0 Mode"]
    #[inline(always)]
    pub fn rdmarg0(self) -> &'a mut W {
        self.variant(BANKMODE_A::RDMARG0)
    }
    #[doc = "Read Mode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(BANKMODE_A::READ)
    }
}
#[doc = "Field `BANK2TRDY` reader - 16:16\\]
Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s)."]
pub type BANK2TRDY_R = crate::BitReader<BANK2TRDY_A>;
#[doc = "16:16\\]
Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BANK2TRDY_A {
    #[doc = "1: Ready"]
    TRUE = 1,
    #[doc = "0: Not ready"]
    FALSE = 0,
}
impl From<BANK2TRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BANK2TRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl BANK2TRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK2TRDY_A {
        match self.bits {
            true => BANK2TRDY_A::TRUE,
            false => BANK2TRDY_A::FALSE,
        }
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == BANK2TRDY_A::TRUE
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == BANK2TRDY_A::FALSE
    }
}
#[doc = "Field `BANK2TRDY` writer - 16:16\\]
Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s)."]
pub type BANK2TRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATMODE_SPEC, BANK2TRDY_A, O>;
impl<'a, const O: u8> BANK2TRDY_W<'a, O> {
    #[doc = "Ready"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(BANK2TRDY_A::TRUE)
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(BANK2TRDY_A::FALSE)
    }
}
#[doc = "Field `BANK1TRDY` reader - 17:17\\]
Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed."]
pub type BANK1TRDY_R = crate::BitReader<BANK1TRDY_A>;
#[doc = "17:17\\]
Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BANK1TRDY_A {
    #[doc = "1: Ready"]
    TRUE = 1,
    #[doc = "0: Not ready"]
    FALSE = 0,
}
impl From<BANK1TRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BANK1TRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl BANK1TRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK1TRDY_A {
        match self.bits {
            true => BANK1TRDY_A::TRUE,
            false => BANK1TRDY_A::FALSE,
        }
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == BANK1TRDY_A::TRUE
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == BANK1TRDY_A::FALSE
    }
}
#[doc = "Field `BANK1TRDY` writer - 17:17\\]
Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed."]
pub type BANK1TRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATMODE_SPEC, BANK1TRDY_A, O>;
impl<'a, const O: u8> BANK1TRDY_W<'a, O> {
    #[doc = "Ready"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(BANK1TRDY_A::TRUE)
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(BANK1TRDY_A::FALSE)
    }
}
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank."]
    #[inline(always)]
    pub fn banknotinrd(&self) -> BANKNOTINRD_R {
        BANKNOTINRD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates mode of bank(s) that are not in READ mode"]
    #[inline(always)]
    pub fn bankmode(&self) -> BANKMODE_R {
        BANKMODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s)."]
    #[inline(always)]
    pub fn bank2trdy(&self) -> BANK2TRDY_R {
        BANK2TRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed."]
    #[inline(always)]
    pub fn bank1trdy(&self) -> BANK1TRDY_R {
        BANK1TRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank."]
    #[inline(always)]
    #[must_use]
    pub fn banknotinrd(&mut self) -> BANKNOTINRD_W<0> {
        BANKNOTINRD_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<5> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates mode of bank(s) that are not in READ mode"]
    #[inline(always)]
    #[must_use]
    pub fn bankmode(&mut self) -> BANKMODE_W<8> {
        BANKMODE_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s)."]
    #[inline(always)]
    #[must_use]
    pub fn bank2trdy(&mut self) -> BANK2TRDY_W<16> {
        BANK2TRDY_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed."]
    #[inline(always)]
    #[must_use]
    pub fn bank1trdy(&mut self) -> BANK1TRDY_W<17> {
        BANK1TRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Status Register Indicates any banks which not in READ mode, and it indicates the mode which the bank(s) are in.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statmode](index.html) module"]
pub struct STATMODE_SPEC;
impl crate::RegisterSpec for STATMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statmode::R](R) reader structure"]
impl crate::Readable for STATMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statmode::W](W) writer structure"]
impl crate::Writable for STATMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATMODE to value 0"]
impl crate::Resettable for STATMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
