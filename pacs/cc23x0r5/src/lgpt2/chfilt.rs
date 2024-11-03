#[doc = "Register `CHFILT` reader"]
pub type R = crate::R<ChfiltSpec>;
#[doc = "Register `CHFILT` writer"]
pub type W = crate::W<ChfiltSpec>;
#[doc = "1:0\\]
Channel filter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "3: Filter is clocked by timer clock."]
    Timerclk = 3,
    #[doc = "2: Filter is clocked by PRECFG.TICKSRC."]
    Ticksrc = 2,
    #[doc = "1: Filter is clocked by system clock."]
    Clk = 1,
    #[doc = "0: Filter is bypassed. No Filter is used."]
    Bypass = 0,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - 1:0\\]
Channel filter mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            3 => Mode::Timerclk,
            2 => Mode::Ticksrc,
            1 => Mode::Clk,
            0 => Mode::Bypass,
            _ => unreachable!(),
        }
    }
    #[doc = "Filter is clocked by timer clock."]
    #[inline(always)]
    pub fn is_timerclk(&self) -> bool {
        *self == Mode::Timerclk
    }
    #[doc = "Filter is clocked by PRECFG.TICKSRC."]
    #[inline(always)]
    pub fn is_ticksrc(&self) -> bool {
        *self == Mode::Ticksrc
    }
    #[doc = "Filter is clocked by system clock."]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Mode::Clk
    }
    #[doc = "Filter is bypassed. No Filter is used."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Mode::Bypass
    }
}
#[doc = "Field `MODE` writer - 1:0\\]
Channel filter mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter is clocked by timer clock."]
    #[inline(always)]
    pub fn timerclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Timerclk)
    }
    #[doc = "Filter is clocked by PRECFG.TICKSRC."]
    #[inline(always)]
    pub fn ticksrc(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ticksrc)
    }
    #[doc = "Filter is clocked by system clock."]
    #[inline(always)]
    pub fn clk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Clk)
    }
    #[doc = "Filter is bypassed. No Filter is used."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bypass)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LOAD` reader - 15:8\\]
The input of the channel filter is passed to the edge detection logic after LOAD + 1 consecutive equal samples."]
pub type LoadR = crate::FieldReader;
#[doc = "Field `LOAD` writer - 15:8\\]
The input of the channel filter is passed to the edge detection logic after LOAD + 1 consecutive equal samples."]
pub type LoadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Channel filter mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The input of the channel filter is passed to the edge detection logic after LOAD + 1 consecutive equal samples."]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(((self.bits >> 8) & 0xff) as u8)
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
Channel filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ChfiltSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ChfiltSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The input of the channel filter is passed to the edge detection logic after LOAD + 1 consecutive equal samples."]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LoadW<ChfiltSpec> {
        LoadW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<ChfiltSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Channel Input Filter This register is used to configure the filter on the channel inputs. The configuration is for all inputs. The filter is enabled when a channel is in capture mode. The input to the filter is passed to the edge detection logic if LOAD + 1 consecutive input samples are equal. The filter functions as a down counter, counting down every input sample. If two consecutive samples are unequal, the filter counter restarts from LOAD. If the filter counter reaches zero, the input signal is valid and passed to the edge detection logic. The channel filter should only be configured while the CTL.MODE = DIS. Configuring the filter while the timer is running can result in unexpected behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chfilt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chfilt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChfiltSpec;
impl crate::RegisterSpec for ChfiltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chfilt::R`](R) reader structure"]
impl crate::Readable for ChfiltSpec {}
#[doc = "`write(|w| ..)` method takes [`chfilt::W`](W) writer structure"]
impl crate::Writable for ChfiltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHFILT to value 0"]
impl crate::Resettable for ChfiltSpec {
    const RESET_VALUE: u32 = 0;
}
