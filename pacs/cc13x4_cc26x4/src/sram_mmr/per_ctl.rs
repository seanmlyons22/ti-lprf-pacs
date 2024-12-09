#[doc = "Register `PER_CTL` reader"]
pub type R = crate::R<PerCtlSpec>;
#[doc = "Register `PER_CTL` writer"]
pub type W = crate::W<PerCtlSpec>;
#[doc = "Field `PER_DEBUG_ENABLE` reader - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
pub type PerDebugEnableR = crate::BitReader;
#[doc = "Field `PER_DEBUG_ENABLE` writer - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
pub type PerDebugEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PER_DISABLE` reader - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
pub type PerDisableR = crate::BitReader;
#[doc = "Field `PER_DISABLE` writer - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
pub type PerDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
    #[inline(always)]
    pub fn per_debug_enable(&self) -> PerDebugEnableR {
        PerDebugEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
    #[inline(always)]
    pub fn per_disable(&self) -> PerDisableR {
        PerDisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
    #[inline(always)]
    #[must_use]
    pub fn per_debug_enable(&mut self) -> PerDebugEnableW<PerCtlSpec> {
        PerDebugEnableW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<PerCtlSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
    #[inline(always)]
    #[must_use]
    pub fn per_disable(&mut self) -> PerDisableW<PerCtlSpec> {
        PerDisableW::new(self, 8)
    }
}
#[doc = "Parity Error Control Parity error check controls\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerCtlSpec;
impl crate::RegisterSpec for PerCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_ctl::R`](R) reader structure"]
impl crate::Readable for PerCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`per_ctl::W`](W) writer structure"]
impl crate::Writable for PerCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER_CTL to value 0"]
impl crate::Resettable for PerCtlSpec {
    const RESET_VALUE: u32 = 0;
}
