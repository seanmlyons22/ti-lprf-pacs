#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `MASTERENABLE` reader - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
pub type MasterenableR = crate::BitReader;
#[doc = "Field `MASTERENABLE` writer - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
pub type MasterenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOCTRL` reader - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
pub type PrtoctrlR = crate::FieldReader;
#[doc = "Field `PRTOCTRL` writer - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
pub type PrtoctrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
    #[inline(always)]
    pub fn masterenable(&self) -> MasterenableR {
        MasterenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[inline(always)]
    pub fn prtoctrl(&self) -> PrtoctrlR {
        PrtoctrlR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
    #[inline(always)]
    #[must_use]
    pub fn masterenable(&mut self) -> MasterenableW<CfgSpec> {
        MasterenableW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CfgSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn prtoctrl(&mut self) -> PrtoctrlW<CfgSpec> {
        PrtoctrlW::new(self, 5)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<CfgSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
