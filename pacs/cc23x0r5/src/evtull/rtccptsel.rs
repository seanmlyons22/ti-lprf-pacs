#[doc = "Register `RTCCPTSEL` reader"]
pub type R = crate::R<RtccptselSpec>;
#[doc = "Register `RTCCPTSEL` writer"]
pub type W = crate::W<RtccptselSpec>;
#[doc = "5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "7: IOC synchronous combined event, controlled by IOC:EVTCFG"]
    AonIocComb = 7,
    #[doc = "6: AON LPCMP interrupt, controlled by SYS0:LPCMPCFG"]
    AonLpmcmpIrq = 6,
    #[doc = "5: DebugSS combined interrupt, interrupt flags can be found here DBGSS:MIS"]
    AonDbgComb = 5,
    #[doc = "4: AON_RTC event, controlled by the RTC:IMASK setting"]
    AonRtcComb = 4,
    #[doc = "3: CKMD combined interrupt request, interrupt flags can be found here CKMD:MIS"]
    AonCkmComb = 3,
    #[doc = "2: PMU combined interrupt request for BATMON, interrupt flags can be found here PMUD:EVENT"]
    AonPmuComb = 2,
    #[doc = "0: Always inactive"]
    None = 0,
}
impl From<Pubid> for u8 {
    #[inline(always)]
    fn from(variant: Pubid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pubid {
    type Ux = u8;
}
impl crate::IsEnum for Pubid {}
#[doc = "Field `PUBID` reader - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type PubidR = crate::FieldReader<Pubid>;
impl PubidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pubid> {
        match self.bits {
            7 => Some(Pubid::AonIocComb),
            6 => Some(Pubid::AonLpmcmpIrq),
            5 => Some(Pubid::AonDbgComb),
            4 => Some(Pubid::AonRtcComb),
            3 => Some(Pubid::AonCkmComb),
            2 => Some(Pubid::AonPmuComb),
            0 => Some(Pubid::None),
            _ => None,
        }
    }
    #[doc = "IOC synchronous combined event, controlled by IOC:EVTCFG"]
    #[inline(always)]
    pub fn is_aon_ioc_comb(&self) -> bool {
        *self == Pubid::AonIocComb
    }
    #[doc = "AON LPCMP interrupt, controlled by SYS0:LPCMPCFG"]
    #[inline(always)]
    pub fn is_aon_lpmcmp_irq(&self) -> bool {
        *self == Pubid::AonLpmcmpIrq
    }
    #[doc = "DebugSS combined interrupt, interrupt flags can be found here DBGSS:MIS"]
    #[inline(always)]
    pub fn is_aon_dbg_comb(&self) -> bool {
        *self == Pubid::AonDbgComb
    }
    #[doc = "AON_RTC event, controlled by the RTC:IMASK setting"]
    #[inline(always)]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == Pubid::AonRtcComb
    }
    #[doc = "CKMD combined interrupt request, interrupt flags can be found here CKMD:MIS"]
    #[inline(always)]
    pub fn is_aon_ckm_comb(&self) -> bool {
        *self == Pubid::AonCkmComb
    }
    #[doc = "PMU combined interrupt request for BATMON, interrupt flags can be found here PMUD:EVENT"]
    #[inline(always)]
    pub fn is_aon_pmu_comb(&self) -> bool {
        *self == Pubid::AonPmuComb
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pubid::None
    }
}
#[doc = "Field `PUBID` writer - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type PubidW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pubid>;
impl<'a, REG> PubidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOC synchronous combined event, controlled by IOC:EVTCFG"]
    #[inline(always)]
    pub fn aon_ioc_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonIocComb)
    }
    #[doc = "AON LPCMP interrupt, controlled by SYS0:LPCMPCFG"]
    #[inline(always)]
    pub fn aon_lpmcmp_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonLpmcmpIrq)
    }
    #[doc = "DebugSS combined interrupt, interrupt flags can be found here DBGSS:MIS"]
    #[inline(always)]
    pub fn aon_dbg_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonDbgComb)
    }
    #[doc = "AON_RTC event, controlled by the RTC:IMASK setting"]
    #[inline(always)]
    pub fn aon_rtc_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonRtcComb)
    }
    #[doc = "CKMD combined interrupt request, interrupt flags can be found here CKMD:MIS"]
    #[inline(always)]
    pub fn aon_ckm_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonCkmComb)
    }
    #[doc = "PMU combined interrupt request for BATMON, interrupt flags can be found here PMUD:EVENT"]
    #[inline(always)]
    pub fn aon_pmu_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonPmuComb)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::None)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn pubid(&self) -> PubidR {
        PubidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn pubid(&mut self) -> PubidW<RtccptselSpec> {
        PubidW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt RTCCPT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccptsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccptsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccptselSpec;
impl crate::RegisterSpec for RtccptselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccptsel::R`](R) reader structure"]
impl crate::Readable for RtccptselSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccptsel::W`](W) writer structure"]
impl crate::Writable for RtccptselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCCPTSEL to value 0"]
impl crate::Resettable for RtccptselSpec {
    const RESET_VALUE: u32 = 0;
}
