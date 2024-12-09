#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AircrSpec>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AircrSpec>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTCLRACTIVE` writer - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
pub type VectclractiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESETREQ` writer - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
pub type SysresetreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESETREQS` reader - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
pub type SysresetreqsR = crate::BitReader;
#[doc = "Field `SYSRESETREQS` writer - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
pub type SysresetreqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIGROUP` reader - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
pub type PrigroupR = crate::FieldReader;
#[doc = "Field `PRIGROUP` writer - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
pub type PrigroupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED11` reader - 12:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `BFHFNMINS` reader - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
pub type BfhfnminsR = crate::BitReader;
#[doc = "Field `BFHFNMINS` writer - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
pub type BfhfnminsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIS` reader - 14:14\\]
Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled."]
pub type PrisR = crate::BitReader;
#[doc = "15:15\\]
Data endianness bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endianess {
    #[doc = "1: Big endian"]
    Big = 1,
    #[doc = "0: Little endian"]
    Little = 0,
}
impl From<Endianess> for bool {
    #[inline(always)]
    fn from(variant: Endianess) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIANESS` reader - 15:15\\]
Data endianness bit"]
pub type EndianessR = crate::BitReader<Endianess>;
impl EndianessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endianess {
        match self.bits {
            true => Endianess::Big,
            false => Endianess::Little,
        }
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == Endianess::Big
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == Endianess::Little
    }
}
#[doc = "Field `VECTKEY` reader - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
pub type VectkeyR = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
pub type VectkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 3 - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
    #[inline(always)]
    pub fn sysresetreqs(&self) -> SysresetreqsR {
        SysresetreqsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    pub fn prigroup(&self) -> PrigroupR {
        PrigroupR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    #[inline(always)]
    pub fn bfhfnmins(&self) -> BfhfnminsR {
        BfhfnminsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled."]
    #[inline(always)]
    pub fn pris(&self) -> PrisR {
        PrisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Data endianness bit"]
    #[inline(always)]
    pub fn endianess(&self) -> EndianessR {
        EndianessR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline(always)]
    pub fn vectkey(&self) -> VectkeyR {
        VectkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AircrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VectclractiveW<AircrSpec> {
        VectclractiveW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SysresetreqW<AircrSpec> {
        SysresetreqW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
System reset request Secure only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreqs(&mut self) -> SysresetreqsW<AircrSpec> {
        SysresetreqsW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<AircrSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PrigroupW<AircrSpec> {
        PrigroupW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception 0x0 BusFault, HardFault, and NMI are Secure. 0x1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmins(&mut self) -> BfhfnminsW<AircrSpec> {
        BfhfnminsW::new(self, 13)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VectkeyW<AircrSpec> {
        VectkeyW::new(self, 16)
    }
}
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AircrSpec;
impl crate::RegisterSpec for AircrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aircr::R`](R) reader structure"]
impl crate::Readable for AircrSpec {}
#[doc = "`write(|w| ..)` method takes [`aircr::W`](W) writer structure"]
impl crate::Writable for AircrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AircrSpec {
    const RESET_VALUE: u32 = 0xfa05_0000;
}
