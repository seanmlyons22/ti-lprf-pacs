#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "3: Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    UpdwnPer = 3,
    #[doc = "2: Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    UpPer = 2,
    #[doc = "1: Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    UpOnce = 1,
    #[doc = "0: Disable timer. Updates to counter, channels, and events stop."]
    Dis = 0,
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
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            3 => Mode::UpdwnPer,
            2 => Mode::UpPer,
            1 => Mode::UpOnce,
            0 => Mode::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn is_updwn_per(&self) -> bool {
        *self == Mode::UpdwnPer
    }
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    #[inline(always)]
    pub fn is_up_per(&self) -> bool {
        *self == Mode::UpPer
    }
    #[doc = "Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn is_up_once(&self) -> bool {
        *self == Mode::UpOnce
    }
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mode::Dis
    }
}
#[doc = "Field `MODE` writer - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn updwn_per(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UpdwnPer)
    }
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    #[inline(always)]
    pub fn up_per(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UpPer)
    }
    #[doc = "Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn up_once(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UpOnce)
    }
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Dis)
    }
}
#[doc = "2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetEn {
    #[doc = "1: TARGET.VALUE"]
    Target = 1,
    #[doc = "0: 65535"]
    CntrMax = 0,
}
impl From<TargetEn> for bool {
    #[inline(always)]
    fn from(variant: TargetEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARGET_EN` reader - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
pub type TargetEnR = crate::BitReader<TargetEn>;
impl TargetEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TargetEn {
        match self.bits {
            true => TargetEn::Target,
            false => TargetEn::CntrMax,
        }
    }
    #[doc = "TARGET.VALUE"]
    #[inline(always)]
    pub fn is_target(&self) -> bool {
        *self == TargetEn::Target
    }
    #[doc = "65535"]
    #[inline(always)]
    pub fn is_cntr_max(&self) -> bool {
        *self == TargetEn::CntrMax
    }
}
#[doc = "Field `TARGET_EN` writer - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
pub type TargetEnW<'a, REG> = crate::BitWriter<'a, REG, TargetEn>;
impl<'a, REG> TargetEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TARGET.VALUE"]
    #[inline(always)]
    pub fn target(self) -> &'a mut crate::W<REG> {
        self.variant(TargetEn::Target)
    }
    #[doc = "65535"]
    #[inline(always)]
    pub fn cntr_max(self) -> &'a mut crate::W<REG> {
        self.variant(TargetEn::CntrMax)
    }
}
#[doc = "Field `CH0_RESET` reader - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
pub type Ch0ResetR = crate::BitReader;
#[doc = "Field `CH0_RESET` writer - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
pub type Ch0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_RESET` reader - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
pub type Ch1ResetR = crate::BitReader;
#[doc = "Field `CH1_RESET` writer - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
pub type Ch1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_RESET` reader - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
pub type Ch2ResetR = crate::BitReader;
#[doc = "Field `CH2_RESET` writer - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
pub type Ch2ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_RESET` reader - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
pub type Ch3ResetR = crate::BitReader;
#[doc = "Field `CH3_RESET` writer - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
pub type Ch3ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline(always)]
    pub fn target_en(&self) -> TargetEnR {
        TargetEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch0_reset(&self) -> Ch0ResetR {
        Ch0ResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch1_reset(&self) -> Ch1ResetR {
        Ch1ResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch2_reset(&self) -> Ch2ResetR {
        Ch2ResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch3_reset(&self) -> Ch3ResetR {
        Ch3ResetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline(always)]
    #[must_use]
    pub fn target_en(&mut self) -> TargetEnW<CtlSpec> {
        TargetEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_reset(&mut self) -> Ch0ResetW<CtlSpec> {
        Ch0ResetW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_reset(&mut self) -> Ch1ResetW<CtlSpec> {
        Ch1ResetW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_reset(&mut self) -> Ch2ResetW<CtlSpec> {
        Ch2ResetW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_reset(&mut self) -> Ch3ResetW<CtlSpec> {
        Ch3ResetW::new(self, 6)
    }
}
#[doc = "Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
