#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AircrSpec>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AircrSpec>;
#[doc = "Field `VECTRESET` reader - 0:0\\]
System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
pub type VectresetR = crate::BitReader;
#[doc = "Field `VECTRESET` writer - 0:0\\]
System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
pub type VectresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTCLRACTIVE` reader - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
pub type VectclractiveR = crate::BitReader;
#[doc = "Field `VECTCLRACTIVE` writer - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
pub type VectclractiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESETREQ` reader - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
pub type SysresetreqR = crate::BitReader;
#[doc = "Field `SYSRESETREQ` writer - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
pub type SysresetreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRIGROUP` reader - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
pub type PrigroupR = crate::FieldReader;
#[doc = "Field `PRIGROUP` writer - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
pub type PrigroupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED11` reader - 14:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - 14:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
#[doc = "Field `ENDIANESS` writer - 15:15\\]
Data endianness bit"]
pub type EndianessW<'a, REG> = crate::BitWriter<'a, REG, Endianess>;
impl<'a, REG> EndianessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(Endianess::Big)
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(Endianess::Little)
    }
}
#[doc = "Field `VECTKEY` reader - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
pub type VectkeyR = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
pub type VectkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[inline(always)]
    pub fn vectreset(&self) -> VectresetR {
        VectresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline(always)]
    pub fn vectclractive(&self) -> VectclractiveR {
        VectclractiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SysresetreqR {
        SysresetreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    pub fn prigroup(&self) -> PrigroupR {
        PrigroupR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 0x0f) as u8)
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
System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vectreset(&mut self) -> VectresetW<AircrSpec> {
        VectresetW::new(self, 0)
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
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<AircrSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PrigroupW<AircrSpec> {
        PrigroupW::new(self, 8)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<AircrSpec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bit 15 - 15:15\\]
Data endianness bit"]
    #[inline(always)]
    #[must_use]
    pub fn endianess(&mut self) -> EndianessW<AircrSpec> {
        EndianessW::new(self, 15)
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
