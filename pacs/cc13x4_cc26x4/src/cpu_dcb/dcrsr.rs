#[doc = "Register `DCRSR` reader"]
pub type R = crate::R<DcrsrSpec>;
#[doc = "Register `DCRSR` writer"]
pub type W = crate::W<DcrsrSpec>;
#[doc = "Field `REGSEL` reader - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
pub type RegselR = crate::FieldReader;
#[doc = "Field `REGSEL` writer - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
pub type RegselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED7` reader - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED7` writer - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `REGWnR` reader - 16:16\\]
Specifies the access type for the transfer"]
pub type RegwnRR = crate::BitReader;
#[doc = "Field `REGWnR` writer - 16:16\\]
Specifies the access type for the transfer"]
pub type RegwnRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
    #[inline(always)]
    pub fn regsel(&self) -> RegselR {
        RegselR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Specifies the access type for the transfer"]
    #[inline(always)]
    pub fn regwn_r(&self) -> RegwnRR {
        RegwnRR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> RegselW<DcrsrSpec> {
        RegselW::new(self, 0)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<DcrsrSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 16 - 16:16\\]
Specifies the access type for the transfer"]
    #[inline(always)]
    #[must_use]
    pub fn regwn_r(&mut self) -> RegwnRW<DcrsrSpec> {
        RegwnRW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<DcrsrSpec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrsrSpec;
impl crate::RegisterSpec for DcrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrsr::R`](R) reader structure"]
impl crate::Readable for DcrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcrsr::W`](W) writer structure"]
impl crate::Writable for DcrsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DcrsrSpec {
    const RESET_VALUE: u32 = 0;
}
