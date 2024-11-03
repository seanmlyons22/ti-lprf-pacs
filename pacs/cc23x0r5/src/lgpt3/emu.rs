#[doc = "Register `EMU` reader"]
pub type R = crate::R<EmuSpec>;
#[doc = "Register `EMU` writer"]
pub type W = crate::W<EmuSpec>;
#[doc = "0:0\\]
Halt LGPT when CPU is halted in debug.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "1: Enable."]
    En = 1,
    #[doc = "0: Disable."]
    Dis = 0,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - 0:0\\]
Halt LGPT when CPU is halted in debug."]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            true => Halt::En,
            false => Halt::Dis,
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Halt::En
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Halt::Dis
    }
}
#[doc = "Field `HALT` writer - 0:0\\]
Halt LGPT when CPU is halted in debug."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::En)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Dis)
    }
}
#[doc = "1:1\\]
Halt control. Configure when the counter shall stop upon CPU halt. This bitfield only applies if HALT = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl {
    #[doc = "1: Zero condition. The counter stops when CNTR = 0."]
    Zercond = 1,
    #[doc = "0: Immediate reaction. The counter stops immediately on debug halt."]
    Immediate = 0,
}
impl From<Ctl> for bool {
    #[inline(always)]
    fn from(variant: Ctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL` reader - 1:1\\]
Halt control. Configure when the counter shall stop upon CPU halt. This bitfield only applies if HALT = 1."]
pub type CtlR = crate::BitReader<Ctl>;
impl CtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl {
        match self.bits {
            true => Ctl::Zercond,
            false => Ctl::Immediate,
        }
    }
    #[doc = "Zero condition. The counter stops when CNTR = 0."]
    #[inline(always)]
    pub fn is_zercond(&self) -> bool {
        *self == Ctl::Zercond
    }
    #[doc = "Immediate reaction. The counter stops immediately on debug halt."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Ctl::Immediate
    }
}
#[doc = "Field `CTL` writer - 1:1\\]
Halt control. Configure when the counter shall stop upon CPU halt. This bitfield only applies if HALT = 1."]
pub type CtlW<'a, REG> = crate::BitWriter<'a, REG, Ctl>;
impl<'a, REG> CtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Zero condition. The counter stops when CNTR = 0."]
    #[inline(always)]
    pub fn zercond(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Zercond)
    }
    #[doc = "Immediate reaction. The counter stops immediately on debug halt."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Immediate)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt LGPT when CPU is halted in debug."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt control. Configure when the counter shall stop upon CPU halt. This bitfield only applies if HALT = 1."]
    #[inline(always)]
    pub fn ctl(&self) -> CtlR {
        CtlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt LGPT when CPU is halted in debug."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<EmuSpec> {
        HaltW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt control. Configure when the counter shall stop upon CPU halt. This bitfield only applies if HALT = 1."]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CtlW<EmuSpec> {
        CtlW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<EmuSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Debug control This register can be used to freeze the timer when CPU halts when HALT is set to 1. When HALT is set to 0, or when the CPU releases debug halt, the filters and edge detection logic is flushed and the timer starts. For setting a predefined output value during a CPU debug halt, PARK, if the timer has this register, should be configured additionally. If this timer does not have the PARK register a predefined output value during CPU halt is not possible.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmuSpec;
impl crate::RegisterSpec for EmuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emu::R`](R) reader structure"]
impl crate::Readable for EmuSpec {}
#[doc = "`write(|w| ..)` method takes [`emu::W`](W) writer structure"]
impl crate::Writable for EmuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMU to value 0"]
impl crate::Resettable for EmuSpec {
    const RESET_VALUE: u32 = 0;
}
