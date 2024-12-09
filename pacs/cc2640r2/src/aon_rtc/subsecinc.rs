#[doc = "Register `SUBSECINC` reader"]
pub type R = crate::R<SubsecincSpec>;
#[doc = "Register `SUBSECINC` writer"]
pub type W = crate::W<SubsecincSpec>;
#[doc = "Field `VALUEINC` reader - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
pub type ValueincR = crate::FieldReader<u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
    #[inline(always)]
    pub fn valueinc(&self) -> ValueincR {
        ValueincR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsecinc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subsecinc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubsecincSpec;
impl crate::RegisterSpec for SubsecincSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subsecinc::R`](R) reader structure"]
impl crate::Readable for SubsecincSpec {}
#[doc = "`write(|w| ..)` method takes [`subsecinc::W`](W) writer structure"]
impl crate::Writable for SubsecincSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBSECINC to value 0x0080_0000"]
impl crate::Resettable for SubsecincSpec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
