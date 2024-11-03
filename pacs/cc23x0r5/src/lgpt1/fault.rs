#[doc = "Register `FAULT` reader"]
pub type R = crate::R<FaultSpec>;
#[doc = "Register `FAULT` writer"]
pub type W = crate::W<FaultSpec>;
#[doc = "1:0\\]
Fault control On active fault input the counter can optionally stop. If the counter stops this is done by hardware, software must then restart the timer if wanted. The fault input overrides channel 0 IOC input when CTL != DIS. This means that channel 0 receives fault as input signal when C0CFG.INPUT = IO and CTL != DIS. CHFILT can be used to avoid glitching on the fault input. Fault is level triggered, the polarity is set by the C0CFG.EDGE field. Here C0CFG.EDGE = RISE gives active high and C0CFG.EDGE = FALL gives active low polarity. Fault is typically used together with PARK to stop the PWM signal to an external motor control circuit safely. Configure PARK to ensure predefined values of the PWM outputs. If CTL != DIS the RIS.FAULT interrupt is set immediately when the fault input is active while CTL.MODE != DIS. The three modes of fault is described below: CTL = IMMEDIATE In this mode the counter stops immediately on an active fault input. This is done by hardware by setting CTL.MODE = DIS. To start the counter software must set CTL.MODE != DIS. When the counter has stopped, the input synchronizers and the channel filter is not running. This means that if RIS.FAULT is cleared it will not be set again while CTL.MODE = DIS. CTL = ZEROCOND In this mode the counter stops when CNTR = 0 after an active fault input. If the RIS.FAULT interrupt has been cleared by software before CNTR = 0, and the fault input is inactive, the counter will continue as normal. When the counter stops on zero, it can be started again by clearing the RIS.FAULT interrupt if the fault input is inactive. To change the counter mode set CTL.MODE = DIS, clear the RIS.FAULT interrupt, then start timer in wanted mode. CTL = IRQ In this mode only the RIS.FAULT flag is set on an active fault input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl {
    #[doc = "3: Interrupt request. Only set RIS.FAULT on active fault."]
    Irq = 3,
    #[doc = "2: Zero condition. The counter stops when CNTR = 0."]
    Zercond = 2,
    #[doc = "1: Immediate reaction. The counter stops immediately on fault."]
    Immediate = 1,
    #[doc = "0: Disable. The timer ignores fault."]
    Dis = 0,
}
impl From<Ctl> for u8 {
    #[inline(always)]
    fn from(variant: Ctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl {
    type Ux = u8;
}
impl crate::IsEnum for Ctl {}
#[doc = "Field `CTL` reader - 1:0\\]
Fault control On active fault input the counter can optionally stop. If the counter stops this is done by hardware, software must then restart the timer if wanted. The fault input overrides channel 0 IOC input when CTL != DIS. This means that channel 0 receives fault as input signal when C0CFG.INPUT = IO and CTL != DIS. CHFILT can be used to avoid glitching on the fault input. Fault is level triggered, the polarity is set by the C0CFG.EDGE field. Here C0CFG.EDGE = RISE gives active high and C0CFG.EDGE = FALL gives active low polarity. Fault is typically used together with PARK to stop the PWM signal to an external motor control circuit safely. Configure PARK to ensure predefined values of the PWM outputs. If CTL != DIS the RIS.FAULT interrupt is set immediately when the fault input is active while CTL.MODE != DIS. The three modes of fault is described below: CTL = IMMEDIATE In this mode the counter stops immediately on an active fault input. This is done by hardware by setting CTL.MODE = DIS. To start the counter software must set CTL.MODE != DIS. When the counter has stopped, the input synchronizers and the channel filter is not running. This means that if RIS.FAULT is cleared it will not be set again while CTL.MODE = DIS. CTL = ZEROCOND In this mode the counter stops when CNTR = 0 after an active fault input. If the RIS.FAULT interrupt has been cleared by software before CNTR = 0, and the fault input is inactive, the counter will continue as normal. When the counter stops on zero, it can be started again by clearing the RIS.FAULT interrupt if the fault input is inactive. To change the counter mode set CTL.MODE = DIS, clear the RIS.FAULT interrupt, then start timer in wanted mode. CTL = IRQ In this mode only the RIS.FAULT flag is set on an active fault input."]
pub type CtlR = crate::FieldReader<Ctl>;
impl CtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl {
        match self.bits {
            3 => Ctl::Irq,
            2 => Ctl::Zercond,
            1 => Ctl::Immediate,
            0 => Ctl::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt request. Only set RIS.FAULT on active fault."]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        *self == Ctl::Irq
    }
    #[doc = "Zero condition. The counter stops when CNTR = 0."]
    #[inline(always)]
    pub fn is_zercond(&self) -> bool {
        *self == Ctl::Zercond
    }
    #[doc = "Immediate reaction. The counter stops immediately on fault."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Ctl::Immediate
    }
    #[doc = "Disable. The timer ignores fault."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ctl::Dis
    }
}
#[doc = "Field `CTL` writer - 1:0\\]
Fault control On active fault input the counter can optionally stop. If the counter stops this is done by hardware, software must then restart the timer if wanted. The fault input overrides channel 0 IOC input when CTL != DIS. This means that channel 0 receives fault as input signal when C0CFG.INPUT = IO and CTL != DIS. CHFILT can be used to avoid glitching on the fault input. Fault is level triggered, the polarity is set by the C0CFG.EDGE field. Here C0CFG.EDGE = RISE gives active high and C0CFG.EDGE = FALL gives active low polarity. Fault is typically used together with PARK to stop the PWM signal to an external motor control circuit safely. Configure PARK to ensure predefined values of the PWM outputs. If CTL != DIS the RIS.FAULT interrupt is set immediately when the fault input is active while CTL.MODE != DIS. The three modes of fault is described below: CTL = IMMEDIATE In this mode the counter stops immediately on an active fault input. This is done by hardware by setting CTL.MODE = DIS. To start the counter software must set CTL.MODE != DIS. When the counter has stopped, the input synchronizers and the channel filter is not running. This means that if RIS.FAULT is cleared it will not be set again while CTL.MODE = DIS. CTL = ZEROCOND In this mode the counter stops when CNTR = 0 after an active fault input. If the RIS.FAULT interrupt has been cleared by software before CNTR = 0, and the fault input is inactive, the counter will continue as normal. When the counter stops on zero, it can be started again by clearing the RIS.FAULT interrupt if the fault input is inactive. To change the counter mode set CTL.MODE = DIS, clear the RIS.FAULT interrupt, then start timer in wanted mode. CTL = IRQ In this mode only the RIS.FAULT flag is set on an active fault input."]
pub type CtlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl, crate::Safe>;
impl<'a, REG> CtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt request. Only set RIS.FAULT on active fault."]
    #[inline(always)]
    pub fn irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Irq)
    }
    #[doc = "Zero condition. The counter stops when CNTR = 0."]
    #[inline(always)]
    pub fn zercond(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Zercond)
    }
    #[doc = "Immediate reaction. The counter stops immediately on fault."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Immediate)
    }
    #[doc = "Disable. The timer ignores fault."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Dis)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Fault control On active fault input the counter can optionally stop. If the counter stops this is done by hardware, software must then restart the timer if wanted. The fault input overrides channel 0 IOC input when CTL != DIS. This means that channel 0 receives fault as input signal when C0CFG.INPUT = IO and CTL != DIS. CHFILT can be used to avoid glitching on the fault input. Fault is level triggered, the polarity is set by the C0CFG.EDGE field. Here C0CFG.EDGE = RISE gives active high and C0CFG.EDGE = FALL gives active low polarity. Fault is typically used together with PARK to stop the PWM signal to an external motor control circuit safely. Configure PARK to ensure predefined values of the PWM outputs. If CTL != DIS the RIS.FAULT interrupt is set immediately when the fault input is active while CTL.MODE != DIS. The three modes of fault is described below: CTL = IMMEDIATE In this mode the counter stops immediately on an active fault input. This is done by hardware by setting CTL.MODE = DIS. To start the counter software must set CTL.MODE != DIS. When the counter has stopped, the input synchronizers and the channel filter is not running. This means that if RIS.FAULT is cleared it will not be set again while CTL.MODE = DIS. CTL = ZEROCOND In this mode the counter stops when CNTR = 0 after an active fault input. If the RIS.FAULT interrupt has been cleared by software before CNTR = 0, and the fault input is inactive, the counter will continue as normal. When the counter stops on zero, it can be started again by clearing the RIS.FAULT interrupt if the fault input is inactive. To change the counter mode set CTL.MODE = DIS, clear the RIS.FAULT interrupt, then start timer in wanted mode. CTL = IRQ In this mode only the RIS.FAULT flag is set on an active fault input."]
    #[inline(always)]
    pub fn ctl(&self) -> CtlR {
        CtlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Fault control On active fault input the counter can optionally stop. If the counter stops this is done by hardware, software must then restart the timer if wanted. The fault input overrides channel 0 IOC input when CTL != DIS. This means that channel 0 receives fault as input signal when C0CFG.INPUT = IO and CTL != DIS. CHFILT can be used to avoid glitching on the fault input. Fault is level triggered, the polarity is set by the C0CFG.EDGE field. Here C0CFG.EDGE = RISE gives active high and C0CFG.EDGE = FALL gives active low polarity. Fault is typically used together with PARK to stop the PWM signal to an external motor control circuit safely. Configure PARK to ensure predefined values of the PWM outputs. If CTL != DIS the RIS.FAULT interrupt is set immediately when the fault input is active while CTL.MODE != DIS. The three modes of fault is described below: CTL = IMMEDIATE In this mode the counter stops immediately on an active fault input. This is done by hardware by setting CTL.MODE = DIS. To start the counter software must set CTL.MODE != DIS. When the counter has stopped, the input synchronizers and the channel filter is not running. This means that if RIS.FAULT is cleared it will not be set again while CTL.MODE = DIS. CTL = ZEROCOND In this mode the counter stops when CNTR = 0 after an active fault input. If the RIS.FAULT interrupt has been cleared by software before CNTR = 0, and the fault input is inactive, the counter will continue as normal. When the counter stops on zero, it can be started again by clearing the RIS.FAULT interrupt if the fault input is inactive. To change the counter mode set CTL.MODE = DIS, clear the RIS.FAULT interrupt, then start timer in wanted mode. CTL = IRQ In this mode only the RIS.FAULT flag is set on an active fault input."]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CtlW<FaultSpec> {
        CtlW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FaultSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Fault This register is used to configure the fault input logic. Primary use scenario is to select CTL before starting the timer. Follow these steps to configure CTL while CTL.MODE is different from DIS: - Set C0CFG.EDGE to NONE. - Configure CTL. - Wait for three system clock periods before setting C0CFG.EDGE different from NONE. These steps prevent fault detection caused by expired signal values in synchronizers and edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultSpec;
impl crate::RegisterSpec for FaultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault::R`](R) reader structure"]
impl crate::Readable for FaultSpec {}
#[doc = "`write(|w| ..)` method takes [`fault::W`](W) writer structure"]
impl crate::Writable for FaultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAULT to value 0"]
impl crate::Resettable for FaultSpec {
    const RESET_VALUE: u32 = 0;
}
