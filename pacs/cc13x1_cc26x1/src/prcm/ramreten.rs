#[doc = "Register `RAMRETEN` reader"]
pub type R = crate::R<RamretenSpec>;
#[doc = "Register `RAMRETEN` writer"]
pub type W = crate::W<RamretenSpec>;
#[doc = "Field `VIMS` reader - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
pub type VimsR = crate::FieldReader;
#[doc = "Field `VIMS` writer - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
pub type VimsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFC` reader - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
pub type RfcR = crate::BitReader;
#[doc = "Field `RFC` writer - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
pub type RfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFCULL` reader - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
pub type RfcullR = crate::BitReader;
#[doc = "Field `RFCULL` writer - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
pub type RfcullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    pub fn vims(&self) -> VimsR {
        VimsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
    #[inline(always)]
    pub fn rfc(&self) -> RfcR {
        RfcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn rfcull(&self) -> RfcullR {
        RfcullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    #[must_use]
    pub fn vims(&mut self) -> VimsW<RamretenSpec> {
        VimsW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
    #[inline(always)]
    #[must_use]
    pub fn rfc(&mut self) -> RfcW<RamretenSpec> {
        RfcW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
RFC ULL is not implemented. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn rfcull(&mut self) -> RfcullW<RamretenSpec> {
        RfcullW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<RamretenSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Memory Retention Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ramreten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ramreten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamretenSpec;
impl crate::RegisterSpec for RamretenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramreten::R`](R) reader structure"]
impl crate::Readable for RamretenSpec {}
#[doc = "`write(|w| ..)` method takes [`ramreten::W`](W) writer structure"]
impl crate::Writable for RamretenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMRETEN to value 0x0b"]
impl crate::Resettable for RamretenSpec {
    const RESET_VALUE: u32 = 0x0b;
}
