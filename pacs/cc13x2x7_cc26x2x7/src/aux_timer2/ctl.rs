#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "3: Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    UPDWN_PER = 3,
    #[doc = "2: Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    UP_PER = 2,
    #[doc = "1: Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    UP_ONCE = 1,
    #[doc = "0: Disable timer. Updates to counter, channels, and events stop."]
    DIS = 0,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            3 => MODE_A::UPDWN_PER,
            2 => MODE_A::UP_PER,
            1 => MODE_A::UP_ONCE,
            0 => MODE_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UPDWN_PER`"]
    #[inline(always)]
    pub fn is_updwn_per(&self) -> bool {
        *self == MODE_A::UPDWN_PER
    }
    #[doc = "Checks if the value of the field is `UP_PER`"]
    #[inline(always)]
    pub fn is_up_per(&self) -> bool {
        *self == MODE_A::UP_PER
    }
    #[doc = "Checks if the value of the field is `UP_ONCE`"]
    #[inline(always)]
    pub fn is_up_once(&self) -> bool {
        *self == MODE_A::UP_ONCE
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MODE_A::DIS
    }
}
#[doc = "Field `MODE` writer - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn updwn_per(self) -> &'a mut W {
        self.variant(MODE_A::UPDWN_PER)
    }
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    #[inline(always)]
    pub fn up_per(self) -> &'a mut W {
        self.variant(MODE_A::UP_PER)
    }
    #[doc = "Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn up_once(self) -> &'a mut W {
        self.variant(MODE_A::UP_ONCE)
    }
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MODE_A::DIS)
    }
}
#[doc = "Field `TARGET_EN` reader - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
pub type TARGET_EN_R = crate::BitReader<TARGET_EN_A>;
#[doc = "2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TARGET_EN_A {
    #[doc = "1: TARGET.VALUE"]
    TARGET = 1,
    #[doc = "0: 65535"]
    CNTR_MAX = 0,
}
impl From<TARGET_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TARGET_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TARGET_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARGET_EN_A {
        match self.bits {
            true => TARGET_EN_A::TARGET,
            false => TARGET_EN_A::CNTR_MAX,
        }
    }
    #[doc = "Checks if the value of the field is `TARGET`"]
    #[inline(always)]
    pub fn is_target(&self) -> bool {
        *self == TARGET_EN_A::TARGET
    }
    #[doc = "Checks if the value of the field is `CNTR_MAX`"]
    #[inline(always)]
    pub fn is_cntr_max(&self) -> bool {
        *self == TARGET_EN_A::CNTR_MAX
    }
}
#[doc = "Field `TARGET_EN` writer - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
pub type TARGET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, TARGET_EN_A, O>;
impl<'a, const O: u8> TARGET_EN_W<'a, O> {
    #[doc = "TARGET.VALUE"]
    #[inline(always)]
    pub fn target(self) -> &'a mut W {
        self.variant(TARGET_EN_A::TARGET)
    }
    #[doc = "65535"]
    #[inline(always)]
    pub fn cntr_max(self) -> &'a mut W {
        self.variant(TARGET_EN_A::CNTR_MAX)
    }
}
#[doc = "Field `CH0_RESET` reader - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
pub type CH0_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CH0_RESET` writer - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
pub type CH0_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CH1_RESET` reader - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
pub type CH1_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CH1_RESET` writer - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
pub type CH1_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CH2_RESET` reader - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
pub type CH2_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CH2_RESET` writer - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
pub type CH2_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CH3_RESET` reader - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
pub type CH3_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CH3_RESET` writer - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
pub type CH3_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline(always)]
    pub fn target_en(&self) -> TARGET_EN_R {
        TARGET_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch0_reset(&self) -> CH0_RESET_R {
        CH0_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch1_reset(&self) -> CH1_RESET_R {
        CH1_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch2_reset(&self) -> CH2_RESET_R {
        CH2_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch3_reset(&self) -> CH3_RESET_R {
        CH3_RESET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline(always)]
    #[must_use]
    pub fn target_en(&mut self) -> TARGET_EN_W<2> {
        TARGET_EN_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_reset(&mut self) -> CH0_RESET_W<3> {
        CH0_RESET_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_reset(&mut self) -> CH1_RESET_W<4> {
        CH1_RESET_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_reset(&mut self) -> CH2_RESET_W<5> {
        CH2_RESET_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_reset(&mut self) -> CH3_RESET_W<6> {
        CH3_RESET_W::new(self)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
