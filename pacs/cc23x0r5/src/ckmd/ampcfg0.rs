#[doc = "Register `AMPCFG0` reader"]
pub type R = crate::R<Ampcfg0Spec>;
#[doc = "Register `AMPCFG0` writer"]
pub type W = crate::W<Ampcfg0Spec>;
#[doc = "4:0\\]
Update rate for the AMPCOMP update rate. Also affects the clock rate for the Amplitude ADC. The update rate is 6MHz / (FSMRATE+1).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsmrate {
    #[doc = "23: 250 kHz"]
    _250k = 23,
    #[doc = "11: 500 kHz"]
    _500k = 11,
    #[doc = "5: 1 MHz"]
    _1m = 5,
    #[doc = "2: 2 MHz"]
    _2m = 2,
    #[doc = "1: 3 MHz"]
    _3m = 1,
    #[doc = "0: 6 MHz"]
    _6m = 0,
}
impl From<Fsmrate> for u8 {
    #[inline(always)]
    fn from(variant: Fsmrate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsmrate {
    type Ux = u8;
}
impl crate::IsEnum for Fsmrate {}
#[doc = "Field `FSMRATE` reader - 4:0\\]
Update rate for the AMPCOMP update rate. Also affects the clock rate for the Amplitude ADC. The update rate is 6MHz / (FSMRATE+1)."]
pub type FsmrateR = crate::FieldReader<Fsmrate>;
impl FsmrateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fsmrate> {
        match self.bits {
            23 => Some(Fsmrate::_250k),
            11 => Some(Fsmrate::_500k),
            5 => Some(Fsmrate::_1m),
            2 => Some(Fsmrate::_2m),
            1 => Some(Fsmrate::_3m),
            0 => Some(Fsmrate::_6m),
            _ => None,
        }
    }
    #[doc = "250 kHz"]
    #[inline(always)]
    pub fn is_250k(&self) -> bool {
        *self == Fsmrate::_250k
    }
    #[doc = "500 kHz"]
    #[inline(always)]
    pub fn is_500k(&self) -> bool {
        *self == Fsmrate::_500k
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == Fsmrate::_1m
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn is_2m(&self) -> bool {
        *self == Fsmrate::_2m
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn is_3m(&self) -> bool {
        *self == Fsmrate::_3m
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn is_6m(&self) -> bool {
        *self == Fsmrate::_6m
    }
}
#[doc = "Field `FSMRATE` writer - 4:0\\]
Update rate for the AMPCOMP update rate. Also affects the clock rate for the Amplitude ADC. The update rate is 6MHz / (FSMRATE+1)."]
pub type FsmrateW<'a, REG> = crate::FieldWriter<'a, REG, 5, Fsmrate>;
impl<'a, REG> FsmrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "250 kHz"]
    #[inline(always)]
    pub fn _250k(self) -> &'a mut crate::W<REG> {
        self.variant(Fsmrate::_250k)
    }
    #[doc = "500 kHz"]
    #[inline(always)]
    pub fn _500k(self) -> &'a mut crate::W<REG> {
        self.variant(Fsmrate::_500k)
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn _1m(self) -> &'a mut crate::W<REG> {
        self.variant(Fsmrate::_1m)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn _2m(self) -> &'a mut crate::W<REG> {
        self.variant(Fsmrate::_2m)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn _3m(self) -> &'a mut crate::W<REG> {
        self.variant(Fsmrate::_3m)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn _6m(self) -> &'a mut crate::W<REG> {
        self.variant(Fsmrate::_6m)
    }
}
#[doc = "Field `INJTIME` reader - 9:5\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles the injection is enabled. The clock speed is defined in FSMRATE. Set to 0 to disable injection."]
pub type InjtimeR = crate::FieldReader;
#[doc = "Field `INJTIME` writer - 9:5\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles the injection is enabled. The clock speed is defined in FSMRATE. Set to 0 to disable injection."]
pub type InjtimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INJWAIT` reader - 14:10\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles to wait after injection is done. The clock speed is defined in FSMRATE."]
pub type InjwaitR = crate::FieldReader;
#[doc = "Field `INJWAIT` writer - 14:10\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles to wait after injection is done. The clock speed is defined in FSMRATE."]
pub type InjwaitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LDOSTART` reader - 19:15\\]
LDO startup time. Number of clock cycles to bypass the LDO resistors for faster startup. Clock frequency defined in FSMRATE."]
pub type LdostartR = crate::FieldReader;
#[doc = "Field `LDOSTART` writer - 19:15\\]
LDO startup time. Number of clock cycles to bypass the LDO resistors for faster startup. Clock frequency defined in FSMRATE."]
pub type LdostartW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADCDLY` reader - 23:20\\]
ADC and PEAKDET startup time. Number of clock cycles to wait after enabling the PEAKDET and ADC before the first measurement. Clock frequency defined in FSMRATE."]
pub type AdcdlyR = crate::FieldReader;
#[doc = "Field `ADCDLY` writer - 23:20\\]
ADC and PEAKDET startup time. Number of clock cycles to wait after enabling the PEAKDET and ADC before the first measurement. Clock frequency defined in FSMRATE."]
pub type AdcdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Q1DLY` reader - 27:24\\]
Q1CAP change delay. Number of clock cycles to wait before changing Q1CAP by one step. Clock frequency defined in FSMRATE."]
pub type Q1dlyR = crate::FieldReader;
#[doc = "Field `Q1DLY` writer - 27:24\\]
Q1CAP change delay. Number of clock cycles to wait before changing Q1CAP by one step. Clock frequency defined in FSMRATE."]
pub type Q1dlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Q2DLY` reader - 31:28\\]
Q2CAP change delay. Number of clock cycles to wait before changing Q2CAP by one step. Clock frequency defined in FSMRATE."]
pub type Q2dlyR = crate::FieldReader;
#[doc = "Field `Q2DLY` writer - 31:28\\]
Q2CAP change delay. Number of clock cycles to wait before changing Q2CAP by one step. Clock frequency defined in FSMRATE."]
pub type Q2dlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Update rate for the AMPCOMP update rate. Also affects the clock rate for the Amplitude ADC. The update rate is 6MHz / (FSMRATE+1)."]
    #[inline(always)]
    pub fn fsmrate(&self) -> FsmrateR {
        FsmrateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles the injection is enabled. The clock speed is defined in FSMRATE. Set to 0 to disable injection."]
    #[inline(always)]
    pub fn injtime(&self) -> InjtimeR {
        InjtimeR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles to wait after injection is done. The clock speed is defined in FSMRATE."]
    #[inline(always)]
    pub fn injwait(&self) -> InjwaitR {
        InjwaitR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
LDO startup time. Number of clock cycles to bypass the LDO resistors for faster startup. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    pub fn ldostart(&self) -> LdostartR {
        LdostartR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
ADC and PEAKDET startup time. Number of clock cycles to wait after enabling the PEAKDET and ADC before the first measurement. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    pub fn adcdly(&self) -> AdcdlyR {
        AdcdlyR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Q1CAP change delay. Number of clock cycles to wait before changing Q1CAP by one step. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    pub fn q1dly(&self) -> Q1dlyR {
        Q1dlyR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Q2CAP change delay. Number of clock cycles to wait before changing Q2CAP by one step. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    pub fn q2dly(&self) -> Q2dlyR {
        Q2dlyR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Update rate for the AMPCOMP update rate. Also affects the clock rate for the Amplitude ADC. The update rate is 6MHz / (FSMRATE+1)."]
    #[inline(always)]
    #[must_use]
    pub fn fsmrate(&mut self) -> FsmrateW<Ampcfg0Spec> {
        FsmrateW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles the injection is enabled. The clock speed is defined in FSMRATE. Set to 0 to disable injection."]
    #[inline(always)]
    #[must_use]
    pub fn injtime(&mut self) -> InjtimeW<Ampcfg0Spec> {
        InjtimeW::new(self, 5)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Inject HFOSC for faster HFXT startup. This value specifies the number of clock cycles to wait after injection is done. The clock speed is defined in FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn injwait(&mut self) -> InjwaitW<Ampcfg0Spec> {
        InjwaitW::new(self, 10)
    }
    #[doc = "Bits 15:19 - 19:15\\]
LDO startup time. Number of clock cycles to bypass the LDO resistors for faster startup. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn ldostart(&mut self) -> LdostartW<Ampcfg0Spec> {
        LdostartW::new(self, 15)
    }
    #[doc = "Bits 20:23 - 23:20\\]
ADC and PEAKDET startup time. Number of clock cycles to wait after enabling the PEAKDET and ADC before the first measurement. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn adcdly(&mut self) -> AdcdlyW<Ampcfg0Spec> {
        AdcdlyW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Q1CAP change delay. Number of clock cycles to wait before changing Q1CAP by one step. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn q1dly(&mut self) -> Q1dlyW<Ampcfg0Spec> {
        Q1dlyW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Q2CAP change delay. Number of clock cycles to wait before changing Q2CAP by one step. Clock frequency defined in FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn q2dly(&mut self) -> Q2dlyW<Ampcfg0Spec> {
        Q2dlyW::new(self, 28)
    }
}
#[doc = "Amplitude Compensation Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ampcfg0Spec;
impl crate::RegisterSpec for Ampcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampcfg0::R`](R) reader structure"]
impl crate::Readable for Ampcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ampcfg0::W`](W) writer structure"]
impl crate::Writable for Ampcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPCFG0 to value 0x0034_8882"]
impl crate::Resettable for Ampcfg0Spec {
    const RESET_VALUE: u32 = 0x0034_8882;
}
