#[doc = "Register `PDCTL1` reader"]
pub type R = crate::R<Pdctl1Spec>;
#[doc = "Register `PDCTL1` writer"]
pub type W = crate::W<Pdctl1Spec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `CPU_ON` reader - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
pub type CpuOnR = crate::BitReader;
#[doc = "Field `CPU_ON` writer - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
pub type CpuOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFC_ON` reader - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
pub type RfcOnR = crate::BitReader;
#[doc = "Field `RFC_ON` writer - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
pub type RfcOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIMS_MODE` reader - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
pub type VimsModeR = crate::FieldReader;
#[doc = "Field `VIMS_MODE` writer - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
pub type VimsModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[inline(always)]
    pub fn cpu_on(&self) -> CpuOnR {
        CpuOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
    #[inline(always)]
    pub fn rfc_on(&self) -> RfcOnR {
        RfcOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    pub fn vims_mode(&self) -> VimsModeR {
        VimsModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_on(&mut self) -> CpuOnW<Pdctl1Spec> {
        CpuOnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_on(&mut self) -> RfcOnW<Pdctl1Spec> {
        RfcOnW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    #[must_use]
    pub fn vims_mode(&mut self) -> VimsModeW<Pdctl1Spec> {
        VimsModeW::new(self, 3)
    }
}
#[doc = "Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdctl1Spec;
impl crate::RegisterSpec for Pdctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdctl1::R`](R) reader structure"]
impl crate::Readable for Pdctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdctl1::W`](W) writer structure"]
impl crate::Writable for Pdctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCTL1 to value 0x0a"]
impl crate::Resettable for Pdctl1Spec {
    const RESET_VALUE: u32 = 0x0a;
}
