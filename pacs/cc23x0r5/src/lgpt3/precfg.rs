#[doc = "Register `PRECFG` reader"]
pub type R = crate::R<PrecfgSpec>;
#[doc = "Register `PRECFG` writer"]
pub type W = crate::W<PrecfgSpec>;
#[doc = "1:0\\]
Prescaler tick source. TICKSRC determines the source which decrements the prescaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ticksrc {
    #[doc = "3: Prescaler is updated at both edges of TICKEN."]
    BothTick = 3,
    #[doc = "2: Prescaler is updated at the falling edge of TICKEN."]
    FallTick = 2,
    #[doc = "1: Prescaler is updated at the rising edge of TICKEN."]
    RiseTick = 1,
    #[doc = "0: Prescaler is updated at the system clock."]
    Clk = 0,
}
impl From<Ticksrc> for u8 {
    #[inline(always)]
    fn from(variant: Ticksrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ticksrc {
    type Ux = u8;
}
impl crate::IsEnum for Ticksrc {}
#[doc = "Field `TICKSRC` reader - 1:0\\]
Prescaler tick source. TICKSRC determines the source which decrements the prescaler."]
pub type TicksrcR = crate::FieldReader<Ticksrc>;
impl TicksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ticksrc {
        match self.bits {
            3 => Ticksrc::BothTick,
            2 => Ticksrc::FallTick,
            1 => Ticksrc::RiseTick,
            0 => Ticksrc::Clk,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler is updated at both edges of TICKEN."]
    #[inline(always)]
    pub fn is_both_tick(&self) -> bool {
        *self == Ticksrc::BothTick
    }
    #[doc = "Prescaler is updated at the falling edge of TICKEN."]
    #[inline(always)]
    pub fn is_fall_tick(&self) -> bool {
        *self == Ticksrc::FallTick
    }
    #[doc = "Prescaler is updated at the rising edge of TICKEN."]
    #[inline(always)]
    pub fn is_rise_tick(&self) -> bool {
        *self == Ticksrc::RiseTick
    }
    #[doc = "Prescaler is updated at the system clock."]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Ticksrc::Clk
    }
}
#[doc = "Field `TICKSRC` writer - 1:0\\]
Prescaler tick source. TICKSRC determines the source which decrements the prescaler."]
pub type TicksrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ticksrc, crate::Safe>;
impl<'a, REG> TicksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler is updated at both edges of TICKEN."]
    #[inline(always)]
    pub fn both_tick(self) -> &'a mut crate::W<REG> {
        self.variant(Ticksrc::BothTick)
    }
    #[doc = "Prescaler is updated at the falling edge of TICKEN."]
    #[inline(always)]
    pub fn fall_tick(self) -> &'a mut crate::W<REG> {
        self.variant(Ticksrc::FallTick)
    }
    #[doc = "Prescaler is updated at the rising edge of TICKEN."]
    #[inline(always)]
    pub fn rise_tick(self) -> &'a mut crate::W<REG> {
        self.variant(Ticksrc::RiseTick)
    }
    #[doc = "Prescaler is updated at the system clock."]
    #[inline(always)]
    pub fn clk(self) -> &'a mut crate::W<REG> {
        self.variant(Ticksrc::Clk)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TICKDIV` reader - 15:8\\]
Tick division. TICKDIV determines the timer clock frequency for the counter, and timer output updates. The timer clock frequency is the clock selected by TICKSRC divided by (TICKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
pub type TickdivR = crate::FieldReader;
#[doc = "Field `TICKDIV` writer - 15:8\\]
Tick division. TICKDIV determines the timer clock frequency for the counter, and timer output updates. The timer clock frequency is the clock selected by TICKSRC divided by (TICKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
pub type TickdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Prescaler tick source. TICKSRC determines the source which decrements the prescaler."]
    #[inline(always)]
    pub fn ticksrc(&self) -> TicksrcR {
        TicksrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Tick division. TICKDIV determines the timer clock frequency for the counter, and timer output updates. The timer clock frequency is the clock selected by TICKSRC divided by (TICKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
    #[inline(always)]
    pub fn tickdiv(&self) -> TickdivR {
        TickdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Prescaler tick source. TICKSRC determines the source which decrements the prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn ticksrc(&mut self) -> TicksrcW<PrecfgSpec> {
        TicksrcW::new(self, 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<PrecfgSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Tick division. TICKDIV determines the timer clock frequency for the counter, and timer output updates. The timer clock frequency is the clock selected by TICKSRC divided by (TICKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
    #[inline(always)]
    #[must_use]
    pub fn tickdiv(&mut self) -> TickdivW<PrecfgSpec> {
        TickdivW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<PrecfgSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Clock Prescaler Configuration This register is used to set the timer clock period. The prescaler is a counter which counts down from the value TICKDIV. When the prescaler counter reaches zero, CNTR is updated. The field TICKDIV effectively divides the prescaler tick source. The timer clock frequency can be calculated as TICKSRC/(TICKDIV+1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrecfgSpec;
impl crate::RegisterSpec for PrecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`precfg::R`](R) reader structure"]
impl crate::Readable for PrecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`precfg::W`](W) writer structure"]
impl crate::Writable for PrecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRECFG to value 0"]
impl crate::Resettable for PrecfgSpec {
    const RESET_VALUE: u32 = 0;
}
