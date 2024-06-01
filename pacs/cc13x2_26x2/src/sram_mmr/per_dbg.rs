#[doc = "Register `PER_DBG` reader"]
pub type R = crate::R<PerDbgSpec>;
#[doc = "Register `PER_DBG` writer"]
pub type W = crate::W<PerDbgSpec>;
#[doc = "Field `PER_DEBUG_ADDR` reader - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
pub type PerDebugAddrR = crate::FieldReader<u32>;
#[doc = "Field `PER_DEBUG_ADDR` writer - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
pub type PerDebugAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
    #[inline(always)]
    pub fn per_debug_addr(&self) -> PerDebugAddrR {
        PerDebugAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
    #[inline(always)]
    #[must_use]
    pub fn per_debug_addr(&mut self) -> PerDebugAddrW<PerDbgSpec> {
        PerDebugAddrW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<PerDbgSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Parity Error Debug Parity error check debug address setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_dbg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_dbg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerDbgSpec;
impl crate::RegisterSpec for PerDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_dbg::R`](R) reader structure"]
impl crate::Readable for PerDbgSpec {}
#[doc = "`write(|w| ..)` method takes [`per_dbg::W`](W) writer structure"]
impl crate::Writable for PerDbgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER_DBG to value 0"]
impl crate::Resettable for PerDbgSpec {
    const RESET_VALUE: u32 = 0;
}
