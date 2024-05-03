#[doc = "Register `OPMODEREQ` reader"]
pub type R = crate::R<OpmodereqSpec>;
#[doc = "Register `OPMODEREQ` writer"]
pub type W = crate::W<OpmodereqSpec>;
#[doc = "1:0\\]
AUX operational mode request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Req {
    #[doc = "3: Powerdown operational mode with wakeup to lowpower mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    Pdlp = 3,
    #[doc = "2: Powerdown operational mode with wakeup to active mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    Pda = 2,
    #[doc = "1: Lowpower operational mode, characterized by: - Powerdown system power supply state (uLDO) request. - SCE clock frequency (SCE_RATE) equals SCLK_MF. - An active wakeup flag does not change operational mode."]
    Lp = 1,
    #[doc = "0: Active operational mode, characterized by: - Active system power supply state (GLDO or DCDC) request. - AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag does not change operational mode."]
    A = 0,
}
impl From<Req> for u8 {
    #[inline(always)]
    fn from(variant: Req) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Req {
    type Ux = u8;
}
impl crate::IsEnum for Req {}
#[doc = "Field `REQ` reader - 1:0\\]
AUX operational mode request."]
pub type ReqR = crate::FieldReader<Req>;
impl ReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Req {
        match self.bits {
            3 => Req::Pdlp,
            2 => Req::Pda,
            1 => Req::Lp,
            0 => Req::A,
            _ => unreachable!(),
        }
    }
    #[doc = "Powerdown operational mode with wakeup to lowpower mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    #[inline(always)]
    pub fn is_pdlp(&self) -> bool {
        *self == Req::Pdlp
    }
    #[doc = "Powerdown operational mode with wakeup to active mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    #[inline(always)]
    pub fn is_pda(&self) -> bool {
        *self == Req::Pda
    }
    #[doc = "Lowpower operational mode, characterized by: - Powerdown system power supply state (uLDO) request. - SCE clock frequency (SCE_RATE) equals SCLK_MF. - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Req::Lp
    }
    #[doc = "Active operational mode, characterized by: - Active system power supply state (GLDO or DCDC) request. - AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Req::A
    }
}
#[doc = "Field `REQ` writer - 1:0\\]
AUX operational mode request."]
pub type ReqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Req, crate::Safe>;
impl<'a, REG> ReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Powerdown operational mode with wakeup to lowpower mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    #[inline(always)]
    pub fn pdlp(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Pdlp)
    }
    #[doc = "Powerdown operational mode with wakeup to active mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    #[inline(always)]
    pub fn pda(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Pda)
    }
    #[doc = "Lowpower operational mode, characterized by: - Powerdown system power supply state (uLDO) request. - SCE clock frequency (SCE_RATE) equals SCLK_MF. - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Lp)
    }
    #[doc = "Active operational mode, characterized by: - Active system power supply state (GLDO or DCDC) request. - AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Req::A)
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
AUX operational mode request."]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 3) as u8)
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
AUX operational mode request."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<OpmodereqSpec> {
        ReqW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<OpmodereqSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opmodereq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opmodereq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpmodereqSpec;
impl crate::RegisterSpec for OpmodereqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opmodereq::R`](R) reader structure"]
impl crate::Readable for OpmodereqSpec {}
#[doc = "`write(|w| ..)` method takes [`opmodereq::W`](W) writer structure"]
impl crate::Writable for OpmodereqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPMODEREQ to value 0"]
impl crate::Resettable for OpmodereqSpec {
    const RESET_VALUE: u32 = 0;
}
