#[doc = "Register `MEMTYPE` reader"]
pub type R = crate::R<MemtypeSpec>;
#[doc = "Register `MEMTYPE` writer"]
pub type W = crate::W<MemtypeSpec>;
#[doc = "Field `SYSMEM` reader - 0:0\\]
System memory present. Indicates whether system memory is present on the bus that connects to the ROM table. 0x0:System memory not present on bus. this is a dedicated debug bus. 0x1:System memory is present on bus."]
pub type SysmemR = crate::BitReader;
#[doc = "Field `SYSMEM` writer - 0:0\\]
System memory present. Indicates whether system memory is present on the bus that connects to the ROM table. 0x0:System memory not present on bus. this is a dedicated debug bus. 0x1:System memory is present on bus."]
pub type SysmemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
System memory present. Indicates whether system memory is present on the bus that connects to the ROM table. 0x0:System memory not present on bus. this is a dedicated debug bus. 0x1:System memory is present on bus."]
    #[inline(always)]
    pub fn sysmem(&self) -> SysmemR {
        SysmemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
System memory present. Indicates whether system memory is present on the bus that connects to the ROM table. 0x0:System memory not present on bus. this is a dedicated debug bus. 0x1:System memory is present on bus."]
    #[inline(always)]
    #[must_use]
    pub fn sysmem(&mut self) -> SysmemW<MemtypeSpec> {
        SysmemW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MemtypeSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Identifies the type of memory present on the bus that connects the DAP to the ROM Table.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemtypeSpec;
impl crate::RegisterSpec for MemtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memtype::R`](R) reader structure"]
impl crate::Readable for MemtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`memtype::W`](W) writer structure"]
impl crate::Writable for MemtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMTYPE to value 0x01"]
impl crate::Resettable for MemtypeSpec {
    const RESET_VALUE: u32 = 0x01;
}