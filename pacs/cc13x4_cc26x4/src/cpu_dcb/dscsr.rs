#[doc = "Register `DSCSR` reader"]
pub type R = crate::R<DscsrSpec>;
#[doc = "Register `DSCSR` writer"]
pub type W = crate::W<DscsrSpec>;
#[doc = "Field `SBRSELEN` reader - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
pub type SbrselenR = crate::BitReader;
#[doc = "Field `SBRSELEN` writer - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
pub type SbrselenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBRSEL` reader - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
pub type SbrselR = crate::BitReader;
#[doc = "Field `SBRSEL` writer - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
pub type SbrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `CDS` reader - 16:16\\]
This field indicates the current Security state of the processor"]
pub type CdsR = crate::BitReader;
#[doc = "Field `CDS` writer - 16:16\\]
This field indicates the current Security state of the processor"]
pub type CdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDSKEY` reader - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
pub type CdskeyR = crate::BitReader;
#[doc = "Field `CDSKEY` writer - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
pub type CdskeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
    #[inline(always)]
    pub fn sbrselen(&self) -> SbrselenR {
        SbrselenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
    #[inline(always)]
    pub fn sbrsel(&self) -> SbrselR {
        SbrselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
This field indicates the current Security state of the processor"]
    #[inline(always)]
    pub fn cds(&self) -> CdsR {
        CdsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
    #[inline(always)]
    pub fn cdskey(&self) -> CdskeyR {
        CdskeyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
    #[inline(always)]
    #[must_use]
    pub fn sbrselen(&mut self) -> SbrselenW<DscsrSpec> {
        SbrselenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
    #[inline(always)]
    #[must_use]
    pub fn sbrsel(&mut self) -> SbrselW<DscsrSpec> {
        SbrselW::new(self, 1)
    }
    #[doc = "Bit 16 - 16:16\\]
This field indicates the current Security state of the processor"]
    #[inline(always)]
    #[must_use]
    pub fn cds(&mut self) -> CdsW<DscsrSpec> {
        CdsW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
    #[inline(always)]
    #[must_use]
    pub fn cdskey(&mut self) -> CdskeyW<DscsrSpec> {
        CdskeyW::new(self, 17)
    }
}
#[doc = "Provides control and status information for Secure debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscsrSpec;
impl crate::RegisterSpec for DscsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscsr::R`](R) reader structure"]
impl crate::Readable for DscsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dscsr::W`](W) writer structure"]
impl crate::Writable for DscsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSCSR to value 0"]
impl crate::Resettable for DscsrSpec {
    const RESET_VALUE: u32 = 0;
}
