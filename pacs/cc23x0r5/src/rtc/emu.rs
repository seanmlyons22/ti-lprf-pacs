#[doc = "Register `EMU` reader"]
pub type R = crate::R<EmuSpec>;
#[doc = "Register `EMU` writer"]
pub type W = crate::W<EmuSpec>;
#[doc = "0:0\\]
Halt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "1: Freeze option. The IP freezes functionality when the core halted input is asserted, and resumes when it is deasserted. The freeze can either be immediate or after the IP has reached a boundary from where it can resume without corruption."]
    Stop = 1,
    #[doc = "0: Free run option. The IP ignores the state of the core halted input."]
    Run = 0,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - 0:0\\]
Halt control."]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            true => Halt::Stop,
            false => Halt::Run,
        }
    }
    #[doc = "Freeze option. The IP freezes functionality when the core halted input is asserted, and resumes when it is deasserted. The freeze can either be immediate or after the IP has reached a boundary from where it can resume without corruption."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Halt::Stop
    }
    #[doc = "Free run option. The IP ignores the state of the core halted input."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Halt::Run
    }
}
#[doc = "Field `HALT` writer - 0:0\\]
Halt control."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Freeze option. The IP freezes functionality when the core halted input is asserted, and resumes when it is deasserted. The freeze can either be immediate or after the IP has reached a boundary from where it can resume without corruption."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Stop)
    }
    #[doc = "Free run option. The IP ignores the state of the core halted input."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Run)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt control."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt control."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<EmuSpec> {
        HaltW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<EmuSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Emulation control register. This register controls the behavior of the IP related to core halted input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
