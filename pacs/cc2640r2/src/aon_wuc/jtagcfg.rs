#[doc = "Register `JTAGCFG` reader"]
pub type R = crate::R<JtagcfgSpec>;
#[doc = "Register `JTAGCFG` writer"]
pub type W = crate::W<JtagcfgSpec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `JTAG_PD_FORCE_ON` reader - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
pub type JtagPdForceOnR = crate::BitReader;
#[doc = "Field `JTAG_PD_FORCE_ON` writer - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
pub type JtagPdForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&self) -> JtagPdForceOnR {
        JtagPdForceOnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<JtagcfgSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_pd_force_on(&mut self) -> JtagPdForceOnW<JtagcfgSpec> {
        JtagPdForceOnW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<JtagcfgSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtagcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtagcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagcfgSpec;
impl crate::RegisterSpec for JtagcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtagcfg::R`](R) reader structure"]
impl crate::Readable for JtagcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`jtagcfg::W`](W) writer structure"]
impl crate::Writable for JtagcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JTAGCFG to value 0x0100"]
impl crate::Resettable for JtagcfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
