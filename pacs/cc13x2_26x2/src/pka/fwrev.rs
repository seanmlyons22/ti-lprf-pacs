#[doc = "Register `FWREV` reader"]
pub type R = crate::R<FwrevSpec>;
#[doc = "Register `FWREV` writer"]
pub type W = crate::W<FwrevSpec>;
#[doc = "Field `RESERVED0` reader - 15:0\\]
Ignore on read"]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `FW_PATCH_LEVEL` reader - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type FwPatchLevelR = crate::FieldReader;
#[doc = "Field `MINOR_FW_REVISION` reader - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
pub type MinorFwRevisionR = crate::FieldReader;
#[doc = "Field `MAJOR_FW_REVISION` reader - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
pub type MajorFwRevisionR = crate::FieldReader;
#[doc = "Field `FW_CAPABILITIES` reader - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
pub type FwCapabilitiesR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn fw_patch_level(&self) -> FwPatchLevelR {
        FwPatchLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
    #[inline(always)]
    pub fn minor_fw_revision(&self) -> MinorFwRevisionR {
        MinorFwRevisionR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
    #[inline(always)]
    pub fn major_fw_revision(&self) -> MajorFwRevisionR {
        MajorFwRevisionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
    #[inline(always)]
    pub fn fw_capabilities(&self) -> FwCapabilitiesR {
        FwCapabilitiesR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. This register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwrev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwrev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwrevSpec;
impl crate::RegisterSpec for FwrevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwrev::R`](R) reader structure"]
impl crate::Readable for FwrevSpec {}
#[doc = "`write(|w| ..)` method takes [`fwrev::W`](W) writer structure"]
impl crate::Writable for FwrevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWREV to value 0x2150_0000"]
impl crate::Resettable for FwrevSpec {
    const RESET_VALUE: u32 = 0x2150_0000;
}
