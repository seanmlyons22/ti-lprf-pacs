#[doc = "Register `ISET` reader"]
pub type R = crate::R<IsetSpec>;
#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Field `HFXTGOOD` writer - 0:0\\]
HFXT good indication. Indicates that HFXT started correctly. The frequency is not necessarily good enough for radio operation. This is only a one-time check at HFXT startup."]
pub type HfxtgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXTFAULT` writer - 1:1\\]
HFXT fault indication. Indicates that HFXT did not start correctly, or its frequency is too low. HFXT will not recover from this fault and has to be restarted. This is only a one-time check at HFXT startup."]
pub type HfxtfaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXTAMPGOOD` writer - 2:2\\]
HFXT amplitude good indication."]
pub type HfxtampgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACKREFLOSS` writer - 3:3\\]
Clock loss indication from the tracking loop. Indicates that the selected reference clock of the tracking loop is lost."]
pub type TrackreflossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACKREFOOR` writer - 4:4\\]
Out-of-range indication from the tracking loop. Indicates that the selected reference clock frequency of the tracking loop is out-of-range."]
pub type TrackrefoorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCCOMPUPD` writer - 5:5\\]
HFXT-ADC comparison update event. Indicates that the HFXT-ADC comparison is done."]
pub type AdccompupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBIASUPD` writer - 6:6\\]
HFXT-ADC BIAS measurement update event. Indicates that the HFXT-ADC BIAS measurement is done."]
pub type AdcbiasupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPEAKUPD` writer - 7:7\\]
HFXT-ADC PEAK measurement update event. Indicates that the HFXT-ADC PEAK measurement is done."]
pub type AdcpeakupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCDONE` writer - 8:8\\]
TDC done event. Indicates that the TDC measurement is done."]
pub type TdcdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFINCUPD` writer - 9:9\\]
LFINC updated. Indicates that a new LFINC measurement value is available in LFCLKSTAT.LFINC."]
pub type LfincupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFCLKGOOD` writer - 10:10\\]
LF clock good. Indicates that the LF clock is good, according to the configuration in LFQUALCTL."]
pub type LfclkgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFCLKOOR` writer - 11:11\\]
LF clock period out-of-range. Indicates that a LF clock period was measured to be out-of-range, according to LFQUALCTL.MAXERR."]
pub type LfclkoorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFCLKLOSS` writer - 12:12\\]
LF clock is lost. Indicates that no LF clock edge occured for ~49us (~1.6 times nominal period). The system will automatically fall-back to generating LFTICK based on CLKULL, to avoid timing corruption. Note that this signal is NOT related to the analog LF clock-loss detector which can reset the device during STANDBY."]
pub type LfclklossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELFEDGE` writer - 13:13\\]
Pre-LF clock edge detect. Indicates that a positive edge occured on the selected pre-LF clock LFCLKSEL.PRE. Can be used by software to confirm that a LF clock source is running and within the expected frequency, before selecting it as the main LF clock source."]
pub type PrelfedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMPCTRLATTARG` writer - 14:14\\]
HFXT Amplitude compensation - controls at target Indicates that the control values configured in HFXTTARG.Q1CAP, HFXTTARG.Q2CAP and HFXTTARG.IREF or HFXTDYN.Q1CAP, HFXTDYN.Q2CAP and HFXTDYN.IREF are reached."]
pub type AmpctrlattargW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMPSETTLED` writer - 15:15\\]
HFXT Amplitude compensation - settled Indicates that the amplitude compensation FSM has reached the SETTLED or TCXOMODE state, and the controls configured in HFXTTARG or HFXTDYN are reached."]
pub type AmpsettledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFGEARRSTRT` writer - 16:16\\]
LFINC filter gearing restart. Indicates that the LFINC filter restarted gearing. Subsequent LFINC estimates may have higher variation."]
pub type LfgearrstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTICK` writer - 17:17\\]
32kHz TICK to RTC and WDT. Either derived from selected LFCLK or generated from CLKULL in absence of LFCLK."]
pub type LftickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
HFXT good indication. Indicates that HFXT started correctly. The frequency is not necessarily good enough for radio operation. This is only a one-time check at HFXT startup."]
    #[inline(always)]
    #[must_use]
    pub fn hfxtgood(&mut self) -> HfxtgoodW<IsetSpec> {
        HfxtgoodW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
HFXT fault indication. Indicates that HFXT did not start correctly, or its frequency is too low. HFXT will not recover from this fault and has to be restarted. This is only a one-time check at HFXT startup."]
    #[inline(always)]
    #[must_use]
    pub fn hfxtfault(&mut self) -> HfxtfaultW<IsetSpec> {
        HfxtfaultW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
HFXT amplitude good indication."]
    #[inline(always)]
    #[must_use]
    pub fn hfxtampgood(&mut self) -> HfxtampgoodW<IsetSpec> {
        HfxtampgoodW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clock loss indication from the tracking loop. Indicates that the selected reference clock of the tracking loop is lost."]
    #[inline(always)]
    #[must_use]
    pub fn trackrefloss(&mut self) -> TrackreflossW<IsetSpec> {
        TrackreflossW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Out-of-range indication from the tracking loop. Indicates that the selected reference clock frequency of the tracking loop is out-of-range."]
    #[inline(always)]
    #[must_use]
    pub fn trackrefoor(&mut self) -> TrackrefoorW<IsetSpec> {
        TrackrefoorW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
HFXT-ADC comparison update event. Indicates that the HFXT-ADC comparison is done."]
    #[inline(always)]
    #[must_use]
    pub fn adccompupd(&mut self) -> AdccompupdW<IsetSpec> {
        AdccompupdW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
HFXT-ADC BIAS measurement update event. Indicates that the HFXT-ADC BIAS measurement is done."]
    #[inline(always)]
    #[must_use]
    pub fn adcbiasupd(&mut self) -> AdcbiasupdW<IsetSpec> {
        AdcbiasupdW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
HFXT-ADC PEAK measurement update event. Indicates that the HFXT-ADC PEAK measurement is done."]
    #[inline(always)]
    #[must_use]
    pub fn adcpeakupd(&mut self) -> AdcpeakupdW<IsetSpec> {
        AdcpeakupdW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
TDC done event. Indicates that the TDC measurement is done."]
    #[inline(always)]
    #[must_use]
    pub fn tdcdone(&mut self) -> TdcdoneW<IsetSpec> {
        TdcdoneW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
LFINC updated. Indicates that a new LFINC measurement value is available in LFCLKSTAT.LFINC."]
    #[inline(always)]
    #[must_use]
    pub fn lfincupd(&mut self) -> LfincupdW<IsetSpec> {
        LfincupdW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
LF clock good. Indicates that the LF clock is good, according to the configuration in LFQUALCTL."]
    #[inline(always)]
    #[must_use]
    pub fn lfclkgood(&mut self) -> LfclkgoodW<IsetSpec> {
        LfclkgoodW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
LF clock period out-of-range. Indicates that a LF clock period was measured to be out-of-range, according to LFQUALCTL.MAXERR."]
    #[inline(always)]
    #[must_use]
    pub fn lfclkoor(&mut self) -> LfclkoorW<IsetSpec> {
        LfclkoorW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
LF clock is lost. Indicates that no LF clock edge occured for ~49us (~1.6 times nominal period). The system will automatically fall-back to generating LFTICK based on CLKULL, to avoid timing corruption. Note that this signal is NOT related to the analog LF clock-loss detector which can reset the device during STANDBY."]
    #[inline(always)]
    #[must_use]
    pub fn lfclkloss(&mut self) -> LfclklossW<IsetSpec> {
        LfclklossW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Pre-LF clock edge detect. Indicates that a positive edge occured on the selected pre-LF clock LFCLKSEL.PRE. Can be used by software to confirm that a LF clock source is running and within the expected frequency, before selecting it as the main LF clock source."]
    #[inline(always)]
    #[must_use]
    pub fn prelfedge(&mut self) -> PrelfedgeW<IsetSpec> {
        PrelfedgeW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
HFXT Amplitude compensation - controls at target Indicates that the control values configured in HFXTTARG.Q1CAP, HFXTTARG.Q2CAP and HFXTTARG.IREF or HFXTDYN.Q1CAP, HFXTDYN.Q2CAP and HFXTDYN.IREF are reached."]
    #[inline(always)]
    #[must_use]
    pub fn ampctrlattarg(&mut self) -> AmpctrlattargW<IsetSpec> {
        AmpctrlattargW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
HFXT Amplitude compensation - settled Indicates that the amplitude compensation FSM has reached the SETTLED or TCXOMODE state, and the controls configured in HFXTTARG or HFXTDYN are reached."]
    #[inline(always)]
    #[must_use]
    pub fn ampsettled(&mut self) -> AmpsettledW<IsetSpec> {
        AmpsettledW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
LFINC filter gearing restart. Indicates that the LFINC filter restarted gearing. Subsequent LFINC estimates may have higher variation."]
    #[inline(always)]
    #[must_use]
    pub fn lfgearrstrt(&mut self) -> LfgearrstrtW<IsetSpec> {
        LfgearrstrtW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
32kHz TICK to RTC and WDT. Either derived from selected LFCLK or generated from CLKULL in absence of LFCLK."]
    #[inline(always)]
    #[must_use]
    pub fn lftick(&mut self) -> LftickW<IsetSpec> {
        LftickW::new(self, 17)
    }
}
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iset::R`](R) reader structure"]
impl crate::Readable for IsetSpec {}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {
    const RESET_VALUE: u32 = 0;
}
