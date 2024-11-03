#[doc = "Register `CH2CFG` reader"]
pub type R = crate::R<Ch2cfgSpec>;
#[doc = "Register `CH2CFG` writer"]
pub type W = crate::W<Ch2cfgSpec>;
#[doc = "0:0\\]
Decides the channel mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "1: channel is in capture mode"]
    Capt = 1,
    #[doc = "0: channel is disabled"]
    Dis = 0,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - 0:0\\]
Decides the channel mode."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            true => Mode::Capt,
            false => Mode::Dis,
        }
    }
    #[doc = "channel is in capture mode"]
    #[inline(always)]
    pub fn is_capt(&self) -> bool {
        *self == Mode::Capt
    }
    #[doc = "channel is disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mode::Dis
    }
}
#[doc = "Field `MODE` writer - 0:0\\]
Decides the channel mode."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "channel is in capture mode"]
    #[inline(always)]
    pub fn capt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Capt)
    }
    #[doc = "channel is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Dis)
    }
}
#[doc = "2:1\\]
Decides the channel input signal's mode. Setting the Value as 2'b11 selects the Rise Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inp {
    #[doc = "2: Capture on both Edge"]
    Both = 2,
    #[doc = "1: Capture on Falling Edge"]
    Fall = 1,
    #[doc = "0: Capture on rising edge"]
    Rise = 0,
}
impl From<Inp> for u8 {
    #[inline(always)]
    fn from(variant: Inp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inp {
    type Ux = u8;
}
impl crate::IsEnum for Inp {}
#[doc = "Field `INP` reader - 2:1\\]
Decides the channel input signal's mode. Setting the Value as 2'b11 selects the Rise Function"]
pub type InpR = crate::FieldReader<Inp>;
impl InpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inp> {
        match self.bits {
            2 => Some(Inp::Both),
            1 => Some(Inp::Fall),
            0 => Some(Inp::Rise),
            _ => None,
        }
    }
    #[doc = "Capture on both Edge"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Inp::Both
    }
    #[doc = "Capture on Falling Edge"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Inp::Fall
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Inp::Rise
    }
}
#[doc = "Field `INP` writer - 2:1\\]
Decides the channel input signal's mode. Setting the Value as 2'b11 selects the Rise Function"]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Inp>;
impl<'a, REG> InpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture on both Edge"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Both)
    }
    #[doc = "Capture on Falling Edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Fall)
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Rise)
    }
}
#[doc = "3:3\\]
When Rearm is enabled the channel remains in continous capture mode. Otherwise it'll be in One shot capture mode. Rearm is only valid for capture mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rearm {
    #[doc = "1: Re arm is enabled"]
    En = 1,
    #[doc = "0: Re Arm is disabled"]
    Dis = 0,
}
impl From<Rearm> for bool {
    #[inline(always)]
    fn from(variant: Rearm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REARM` reader - 3:3\\]
When Rearm is enabled the channel remains in continous capture mode. Otherwise it'll be in One shot capture mode. Rearm is only valid for capture mode."]
pub type RearmR = crate::BitReader<Rearm>;
impl RearmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rearm {
        match self.bits {
            true => Rearm::En,
            false => Rearm::Dis,
        }
    }
    #[doc = "Re arm is enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rearm::En
    }
    #[doc = "Re Arm is disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rearm::Dis
    }
}
#[doc = "Field `REARM` writer - 3:3\\]
When Rearm is enabled the channel remains in continous capture mode. Otherwise it'll be in One shot capture mode. Rearm is only valid for capture mode."]
pub type RearmW<'a, REG> = crate::BitWriter<'a, REG, Rearm>;
impl<'a, REG> RearmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Re arm is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rearm::En)
    }
    #[doc = "Re Arm is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rearm::Dis)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Decides the channel mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Decides the channel input signal's mode. Setting the Value as 2'b11 selects the Rise Function"]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
When Rearm is enabled the channel remains in continous capture mode. Otherwise it'll be in One shot capture mode. Rearm is only valid for capture mode."]
    #[inline(always)]
    pub fn rearm(&self) -> RearmR {
        RearmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Decides the channel mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Ch2cfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Decides the channel input signal's mode. Setting the Value as 2'b11 selects the Rise Function"]
    #[inline(always)]
    #[must_use]
    pub fn inp(&mut self) -> InpW<Ch2cfgSpec> {
        InpW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
When Rearm is enabled the channel remains in continous capture mode. Otherwise it'll be in One shot capture mode. Rearm is only valid for capture mode."]
    #[inline(always)]
    #[must_use]
    pub fn rearm(&mut self) -> RearmW<Ch2cfgSpec> {
        RearmW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Ch2cfgSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Systimer channel 2 configuration. This channel works in 250ns based capture and compare operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2cfgSpec;
impl crate::RegisterSpec for Ch2cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cfg::R`](R) reader structure"]
impl crate::Readable for Ch2cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2cfg::W`](W) writer structure"]
impl crate::Writable for Ch2cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2CFG to value 0"]
impl crate::Resettable for Ch2cfgSpec {
    const RESET_VALUE: u32 = 0;
}
