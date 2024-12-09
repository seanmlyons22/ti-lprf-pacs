#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CYCCNTENA` reader - 0:0\\]
Enables CYCCNT"]
pub type CyccntenaR = crate::BitReader;
#[doc = "Field `POSTPRESET` reader - 4:1\\]
Reload value for the POSTCNT counter"]
pub type PostpresetR = crate::FieldReader;
#[doc = "Field `POSTINIT` reader - 8:5\\]
Initial value for the POSTCNT counter"]
pub type PostinitR = crate::FieldReader;
#[doc = "Field `CYCTAP` reader - 9:9\\]
Selects the position of the POSTCNT tap on the CYCCNT counter"]
pub type CyctapR = crate::BitReader;
#[doc = "Field `SYNCTAP` reader - 11:10\\]
Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
pub type SynctapR = crate::FieldReader;
#[doc = "Field `PCSAMPLENA` reader - 12:12\\]
Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
pub type PcsamplenaR = crate::BitReader;
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader;
#[doc = "Field `EXTTRCENA` reader - 16:16\\]
Enables generation of Exception Trace packets"]
pub type ExttrcenaR = crate::BitReader;
#[doc = "Field `CPIEVTENA` reader - 17:17\\]
Enables DWT_CPICNT counter"]
pub type CpievtenaR = crate::BitReader;
#[doc = "Field `EXCEVTENA` reader - 18:18\\]
Enables DWT_EXCCNT counter"]
pub type ExcevtenaR = crate::BitReader;
#[doc = "Field `SLEEPEVTENA` reader - 19:19\\]
Enable DWT_SLEEPCNT counter"]
pub type SleepevtenaR = crate::BitReader;
#[doc = "Field `LSUEVTENA` reader - 20:20\\]
Enables DWT_LSUCNT counter"]
pub type LsuevtenaR = crate::BitReader;
#[doc = "Field `FOLDEVTENA` reader - 21:21\\]
Enables DWT_FOLDCNT counter"]
pub type FoldevtenaR = crate::BitReader;
#[doc = "Field `CYCEVTENA` reader - 22:22\\]
Enables Event Counter packet generation on POSTCNT underflow"]
pub type CycevtenaR = crate::BitReader;
#[doc = "Field `CYCDISS` reader - 23:23\\]
Controls whether the cycle counter is disabled in Secure state"]
pub type CycdissR = crate::BitReader;
#[doc = "Field `NOPRFCNT` reader - 24:24\\]
Indicates whether the implementation does not include the profiling counters"]
pub type NoprfcntR = crate::BitReader;
#[doc = "Field `NOCYCCNT` reader - 25:25\\]
Indicates whether the implementation does not include a cycle counter"]
pub type NocyccntR = crate::BitReader;
#[doc = "Field `NOEXTTRIG` reader - 26:26\\]
Reserved, RAZ"]
pub type NoexttrigR = crate::BitReader;
#[doc = "Field `NOTRCPKT` reader - 27:27\\]
Indicates whether the implementation does not support trace"]
pub type NotrcpktR = crate::BitReader;
#[doc = "Field `NUMCOMP` reader - 31:28\\]
Number of DWT comparators implemented"]
pub type NumcompR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables CYCCNT"]
    #[inline(always)]
    pub fn cyccntena(&self) -> CyccntenaR {
        CyccntenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for the POSTCNT counter"]
    #[inline(always)]
    pub fn postpreset(&self) -> PostpresetR {
        PostpresetR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Initial value for the POSTCNT counter"]
    #[inline(always)]
    pub fn postinit(&self) -> PostinitR {
        PostinitR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects the position of the POSTCNT tap on the CYCCNT counter"]
    #[inline(always)]
    pub fn cyctap(&self) -> CyctapR {
        CyctapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
    #[inline(always)]
    pub fn synctap(&self) -> SynctapR {
        SynctapR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
    #[inline(always)]
    pub fn pcsamplena(&self) -> PcsamplenaR {
        PcsamplenaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables generation of Exception Trace packets"]
    #[inline(always)]
    pub fn exttrcena(&self) -> ExttrcenaR {
        ExttrcenaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enables DWT_CPICNT counter"]
    #[inline(always)]
    pub fn cpievtena(&self) -> CpievtenaR {
        CpievtenaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables DWT_EXCCNT counter"]
    #[inline(always)]
    pub fn excevtena(&self) -> ExcevtenaR {
        ExcevtenaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DWT_SLEEPCNT counter"]
    #[inline(always)]
    pub fn sleepevtena(&self) -> SleepevtenaR {
        SleepevtenaR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Enables DWT_LSUCNT counter"]
    #[inline(always)]
    pub fn lsuevtena(&self) -> LsuevtenaR {
        LsuevtenaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Enables DWT_FOLDCNT counter"]
    #[inline(always)]
    pub fn foldevtena(&self) -> FoldevtenaR {
        FoldevtenaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Event Counter packet generation on POSTCNT underflow"]
    #[inline(always)]
    pub fn cycevtena(&self) -> CycevtenaR {
        CycevtenaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Controls whether the cycle counter is disabled in Secure state"]
    #[inline(always)]
    pub fn cycdiss(&self) -> CycdissR {
        CycdissR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates whether the implementation does not include the profiling counters"]
    #[inline(always)]
    pub fn noprfcnt(&self) -> NoprfcntR {
        NoprfcntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates whether the implementation does not include a cycle counter"]
    #[inline(always)]
    pub fn nocyccnt(&self) -> NocyccntR {
        NocyccntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved, RAZ"]
    #[inline(always)]
    pub fn noexttrig(&self) -> NoexttrigR {
        NoexttrigR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Indicates whether the implementation does not support trace"]
    #[inline(always)]
    pub fn notrcpkt(&self) -> NotrcpktR {
        NotrcpktR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Number of DWT comparators implemented"]
    #[inline(always)]
    pub fn numcomp(&self) -> NumcompR {
        NumcompR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides configuration and status information for the DWT unit, and used to control features of the unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
