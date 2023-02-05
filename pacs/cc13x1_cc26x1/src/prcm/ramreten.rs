#[doc = "Register `RAMRETEN` reader"]
pub struct R(crate::R<RAMRETEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMRETEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMRETEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMRETEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMRETEN` writer"]
pub struct W(crate::W<RAMRETEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMRETEN_SPEC>;
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
impl From<crate::W<RAMRETEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMRETEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VIMS` reader - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
pub type VIMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIMS` writer - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
pub type VIMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAMRETEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `RFC` reader - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
pub type RFC_R = crate::BitReader<bool>;
#[doc = "Field `RFC` writer - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
pub type RFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMRETEN_SPEC, bool, O>;
#[doc = "Field `RFCULL` reader - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
pub type RFCULL_R = crate::BitReader<bool>;
#[doc = "Field `RFCULL` writer - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
pub type RFCULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMRETEN_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAMRETEN_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    pub fn vims(&self) -> VIMS_R {
        VIMS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn rfcull(&self) -> RFCULL_R {
        RFCULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    #[must_use]
    pub fn vims(&mut self) -> VIMS_W<0> {
        VIMS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
    #[inline(always)]
    #[must_use]
    pub fn rfc(&mut self) -> RFC_W<2> {
        RFC_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn rfcull(&mut self) -> RFCULL_W<3> {
        RFCULL_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Retention Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramreten](index.html) module"]
pub struct RAMRETEN_SPEC;
impl crate::RegisterSpec for RAMRETEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramreten::R](R) reader structure"]
impl crate::Readable for RAMRETEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramreten::W](W) writer structure"]
impl crate::Writable for RAMRETEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMRETEN to value 0x0b"]
impl crate::Resettable for RAMRETEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
