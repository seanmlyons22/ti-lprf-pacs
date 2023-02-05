#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCCNTENA` reader - 0:0\\]
Enables CYCCNT"]
pub type CYCCNTENA_R = crate::BitReader<bool>;
#[doc = "Field `CYCCNTENA` writer - 0:0\\]
Enables CYCCNT"]
pub type CYCCNTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `POSTPRESET` reader - 4:1\\]
Reload value for the POSTCNT counter"]
pub type POSTPRESET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSTPRESET` writer - 4:1\\]
Reload value for the POSTCNT counter"]
pub type POSTPRESET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `POSTINIT` reader - 8:5\\]
Initial value for the POSTCNT counter"]
pub type POSTINIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSTINIT` writer - 8:5\\]
Initial value for the POSTCNT counter"]
pub type POSTINIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CYCTAP` reader - 9:9\\]
Selects the position of the POSTCNT tap on the CYCCNT counter"]
pub type CYCTAP_R = crate::BitReader<bool>;
#[doc = "Field `CYCTAP` writer - 9:9\\]
Selects the position of the POSTCNT tap on the CYCCNT counter"]
pub type CYCTAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SYNCTAP` reader - 11:10\\]
Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
pub type SYNCTAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCTAP` writer - 11:10\\]
Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
pub type SYNCTAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCSAMPLENA` reader - 12:12\\]
Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
pub type PCSAMPLENA_R = crate::BitReader<bool>;
#[doc = "Field `PCSAMPLENA` writer - 12:12\\]
Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
pub type PCSAMPLENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED13` writer - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `EXTTRCENA` reader - 16:16\\]
Enables generation of Exception Trace packets"]
pub type EXTTRCENA_R = crate::BitReader<bool>;
#[doc = "Field `EXTTRCENA` writer - 16:16\\]
Enables generation of Exception Trace packets"]
pub type EXTTRCENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CPIEVTENA` reader - 17:17\\]
Enables DWT_CPICNT counter"]
pub type CPIEVTENA_R = crate::BitReader<bool>;
#[doc = "Field `CPIEVTENA` writer - 17:17\\]
Enables DWT_CPICNT counter"]
pub type CPIEVTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EXCEVTENA` reader - 18:18\\]
Enables DWT_EXCCNT counter"]
pub type EXCEVTENA_R = crate::BitReader<bool>;
#[doc = "Field `EXCEVTENA` writer - 18:18\\]
Enables DWT_EXCCNT counter"]
pub type EXCEVTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SLEEPEVTENA` reader - 19:19\\]
Enable DWT_SLEEPCNT counter"]
pub type SLEEPEVTENA_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPEVTENA` writer - 19:19\\]
Enable DWT_SLEEPCNT counter"]
pub type SLEEPEVTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LSUEVTENA` reader - 20:20\\]
Enables DWT_LSUCNT counter"]
pub type LSUEVTENA_R = crate::BitReader<bool>;
#[doc = "Field `LSUEVTENA` writer - 20:20\\]
Enables DWT_LSUCNT counter"]
pub type LSUEVTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FOLDEVTENA` reader - 21:21\\]
Enables DWT_FOLDCNT counter"]
pub type FOLDEVTENA_R = crate::BitReader<bool>;
#[doc = "Field `FOLDEVTENA` writer - 21:21\\]
Enables DWT_FOLDCNT counter"]
pub type FOLDEVTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CYCEVTENA` reader - 22:22\\]
Enables Event Counter packet generation on POSTCNT underflow"]
pub type CYCEVTENA_R = crate::BitReader<bool>;
#[doc = "Field `CYCEVTENA` writer - 22:22\\]
Enables Event Counter packet generation on POSTCNT underflow"]
pub type CYCEVTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CYCDISS` reader - 23:23\\]
Controls whether the cycle counter is disabled in Secure state"]
pub type CYCDISS_R = crate::BitReader<bool>;
#[doc = "Field `CYCDISS` writer - 23:23\\]
Controls whether the cycle counter is disabled in Secure state"]
pub type CYCDISS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOPRFCNT` reader - 24:24\\]
Indicates whether the implementation does not include the profiling counters"]
pub type NOPRFCNT_R = crate::BitReader<bool>;
#[doc = "Field `NOPRFCNT` writer - 24:24\\]
Indicates whether the implementation does not include the profiling counters"]
pub type NOPRFCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOCYCCNT` reader - 25:25\\]
Indicates whether the implementation does not include a cycle counter"]
pub type NOCYCCNT_R = crate::BitReader<bool>;
#[doc = "Field `NOCYCCNT` writer - 25:25\\]
Indicates whether the implementation does not include a cycle counter"]
pub type NOCYCCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOEXTTRIG` reader - 26:26\\]
Reserved, RAZ"]
pub type NOEXTTRIG_R = crate::BitReader<bool>;
#[doc = "Field `NOEXTTRIG` writer - 26:26\\]
Reserved, RAZ"]
pub type NOEXTTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOTRCPKT` reader - 27:27\\]
Indicates whether the implementation does not support trace"]
pub type NOTRCPKT_R = crate::BitReader<bool>;
#[doc = "Field `NOTRCPKT` writer - 27:27\\]
Indicates whether the implementation does not support trace"]
pub type NOTRCPKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NUMCOMP` reader - 31:28\\]
Number of DWT comparators implemented"]
pub type NUMCOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMCOMP` writer - 31:28\\]
Number of DWT comparators implemented"]
pub type NUMCOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables CYCCNT"]
    #[inline(always)]
    pub fn cyccntena(&self) -> CYCCNTENA_R {
        CYCCNTENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for the POSTCNT counter"]
    #[inline(always)]
    pub fn postpreset(&self) -> POSTPRESET_R {
        POSTPRESET_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Initial value for the POSTCNT counter"]
    #[inline(always)]
    pub fn postinit(&self) -> POSTINIT_R {
        POSTINIT_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects the position of the POSTCNT tap on the CYCCNT counter"]
    #[inline(always)]
    pub fn cyctap(&self) -> CYCTAP_R {
        CYCTAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
    #[inline(always)]
    pub fn synctap(&self) -> SYNCTAP_R {
        SYNCTAP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
    #[inline(always)]
    pub fn pcsamplena(&self) -> PCSAMPLENA_R {
        PCSAMPLENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables generation of Exception Trace packets"]
    #[inline(always)]
    pub fn exttrcena(&self) -> EXTTRCENA_R {
        EXTTRCENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enables DWT_CPICNT counter"]
    #[inline(always)]
    pub fn cpievtena(&self) -> CPIEVTENA_R {
        CPIEVTENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables DWT_EXCCNT counter"]
    #[inline(always)]
    pub fn excevtena(&self) -> EXCEVTENA_R {
        EXCEVTENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DWT_SLEEPCNT counter"]
    #[inline(always)]
    pub fn sleepevtena(&self) -> SLEEPEVTENA_R {
        SLEEPEVTENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Enables DWT_LSUCNT counter"]
    #[inline(always)]
    pub fn lsuevtena(&self) -> LSUEVTENA_R {
        LSUEVTENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Enables DWT_FOLDCNT counter"]
    #[inline(always)]
    pub fn foldevtena(&self) -> FOLDEVTENA_R {
        FOLDEVTENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Event Counter packet generation on POSTCNT underflow"]
    #[inline(always)]
    pub fn cycevtena(&self) -> CYCEVTENA_R {
        CYCEVTENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Controls whether the cycle counter is disabled in Secure state"]
    #[inline(always)]
    pub fn cycdiss(&self) -> CYCDISS_R {
        CYCDISS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates whether the implementation does not include the profiling counters"]
    #[inline(always)]
    pub fn noprfcnt(&self) -> NOPRFCNT_R {
        NOPRFCNT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates whether the implementation does not include a cycle counter"]
    #[inline(always)]
    pub fn nocyccnt(&self) -> NOCYCCNT_R {
        NOCYCCNT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved, RAZ"]
    #[inline(always)]
    pub fn noexttrig(&self) -> NOEXTTRIG_R {
        NOEXTTRIG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Indicates whether the implementation does not support trace"]
    #[inline(always)]
    pub fn notrcpkt(&self) -> NOTRCPKT_R {
        NOTRCPKT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Number of DWT comparators implemented"]
    #[inline(always)]
    pub fn numcomp(&self) -> NUMCOMP_R {
        NUMCOMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables CYCCNT"]
    #[inline(always)]
    #[must_use]
    pub fn cyccntena(&mut self) -> CYCCNTENA_W<0> {
        CYCCNTENA_W::new(self)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for the POSTCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn postpreset(&mut self) -> POSTPRESET_W<1> {
        POSTPRESET_W::new(self)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Initial value for the POSTCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn postinit(&mut self) -> POSTINIT_W<5> {
        POSTINIT_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects the position of the POSTCNT tap on the CYCCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn cyctap(&mut self) -> CYCTAP_W<9> {
        CYCTAP_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
    #[inline(always)]
    #[must_use]
    pub fn synctap(&mut self) -> SYNCTAP_W<10> {
        SYNCTAP_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
    #[inline(always)]
    #[must_use]
    pub fn pcsamplena(&mut self) -> PCSAMPLENA_W<12> {
        PCSAMPLENA_W::new(self)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables generation of Exception Trace packets"]
    #[inline(always)]
    #[must_use]
    pub fn exttrcena(&mut self) -> EXTTRCENA_W<16> {
        EXTTRCENA_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Enables DWT_CPICNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn cpievtena(&mut self) -> CPIEVTENA_W<17> {
        CPIEVTENA_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables DWT_EXCCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn excevtena(&mut self) -> EXCEVTENA_W<18> {
        EXCEVTENA_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DWT_SLEEPCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn sleepevtena(&mut self) -> SLEEPEVTENA_W<19> {
        SLEEPEVTENA_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Enables DWT_LSUCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn lsuevtena(&mut self) -> LSUEVTENA_W<20> {
        LSUEVTENA_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Enables DWT_FOLDCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn foldevtena(&mut self) -> FOLDEVTENA_W<21> {
        FOLDEVTENA_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Event Counter packet generation on POSTCNT underflow"]
    #[inline(always)]
    #[must_use]
    pub fn cycevtena(&mut self) -> CYCEVTENA_W<22> {
        CYCEVTENA_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Controls whether the cycle counter is disabled in Secure state"]
    #[inline(always)]
    #[must_use]
    pub fn cycdiss(&mut self) -> CYCDISS_W<23> {
        CYCDISS_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates whether the implementation does not include the profiling counters"]
    #[inline(always)]
    #[must_use]
    pub fn noprfcnt(&mut self) -> NOPRFCNT_W<24> {
        NOPRFCNT_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates whether the implementation does not include a cycle counter"]
    #[inline(always)]
    #[must_use]
    pub fn nocyccnt(&mut self) -> NOCYCCNT_W<25> {
        NOCYCCNT_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved, RAZ"]
    #[inline(always)]
    #[must_use]
    pub fn noexttrig(&mut self) -> NOEXTTRIG_W<26> {
        NOEXTTRIG_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Indicates whether the implementation does not support trace"]
    #[inline(always)]
    #[must_use]
    pub fn notrcpkt(&mut self) -> NOTRCPKT_W<27> {
        NOTRCPKT_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Number of DWT comparators implemented"]
    #[inline(always)]
    #[must_use]
    pub fn numcomp(&mut self) -> NUMCOMP_W<28> {
        NUMCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides configuration and status information for the DWT unit, and used to control features of the unit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
