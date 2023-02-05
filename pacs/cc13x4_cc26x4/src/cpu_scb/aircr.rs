#[doc = "Register `AIRCR` reader"]
pub struct R(crate::R<AIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIRCR` writer"]
pub struct W(crate::W<AIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIRCR_SPEC>;
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
impl From<crate::W<AIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `VECTCLRACTIVE` reader - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
pub type VECTCLRACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `VECTCLRACTIVE` writer - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
pub type VECTCLRACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `SYSRESETREQ` reader - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
pub type SYSRESETREQ_R = crate::BitReader<bool>;
#[doc = "Field `SYSRESETREQ` writer - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
pub type SYSRESETREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `SYSRESETREQS` reader - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
pub type SYSRESETREQS_R = crate::BitReader<bool>;
#[doc = "Field `SYSRESETREQS` writer - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
pub type SYSRESETREQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRIGROUP` reader - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
pub type PRIGROUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIGROUP` writer - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
pub type PRIGROUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED11` reader - 12:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED11` writer - 12:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BFHFNMINS` reader - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
pub type BFHFNMINS_R = crate::BitReader<bool>;
#[doc = "Field `BFHFNMINS` writer - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
pub type BFHFNMINS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `PRIS` reader - 14:14\\]
Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled."]
pub type PRIS_R = crate::BitReader<bool>;
#[doc = "Field `PRIS` writer - 14:14\\]
Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled."]
pub type PRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `ENDIANESS` reader - 15:15\\]
Data endianness bit"]
pub type ENDIANESS_R = crate::BitReader<ENDIANESS_A>;
#[doc = "15:15\\]
Data endianness bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIANESS_A {
    #[doc = "1: Big endian"]
    BIG = 1,
    #[doc = "0: Little endian"]
    LITTLE = 0,
}
impl From<ENDIANESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANESS_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIANESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANESS_A {
        match self.bits {
            true => ENDIANESS_A::BIG,
            false => ENDIANESS_A::LITTLE,
        }
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIANESS_A::BIG
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIANESS_A::LITTLE
    }
}
#[doc = "Field `ENDIANESS` writer - 15:15\\]
Data endianness bit"]
pub type ENDIANESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, ENDIANESS_A, O>;
impl<'a, const O: u8> ENDIANESS_W<'a, O> {
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIANESS_A::BIG)
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIANESS_A::LITTLE)
    }
}
#[doc = "Field `VECTKEY` reader - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
pub type VECTKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VECTKEY` writer - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
pub type VECTKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline(always)]
    pub fn vectclractive(&self) -> VECTCLRACTIVE_R {
        VECTCLRACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
    #[inline(always)]
    pub fn sysresetreqs(&self) -> SYSRESETREQS_R {
        SYSRESETREQS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    #[inline(always)]
    pub fn bfhfnmins(&self) -> BFHFNMINS_R {
        BFHFNMINS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled."]
    #[inline(always)]
    pub fn pris(&self) -> PRIS_R {
        PRIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Data endianness bit"]
    #[inline(always)]
    pub fn endianess(&self) -> ENDIANESS_R {
        ENDIANESS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
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
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<1> {
        VECTCLRACTIVE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<2> {
        SYSRESETREQ_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreqs(&mut self) -> SYSRESETREQS_W<3> {
        SYSRESETREQS_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PRIGROUP_W<8> {
        PRIGROUP_W::new(self)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmins(&mut self) -> BFHFNMINS_W<13> {
        BFHFNMINS_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pris(&mut self) -> PRIS_W<14> {
        PRIS_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Data endianness bit"]
    #[inline(always)]
    #[must_use]
    pub fn endianess(&mut self) -> ENDIANESS_W<15> {
        ENDIANESS_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<16> {
        VECTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](index.html) module"]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aircr::R](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aircr::W](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0000;
}
