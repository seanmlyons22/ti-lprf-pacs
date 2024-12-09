#[doc = "Register `NVMNSADDR` reader"]
pub type R = crate::R<NvmnsaddrSpec>;
#[doc = "Register `NVMNSADDR` writer"]
pub type W = crate::W<NvmnsaddrSpec>;
#[doc = "Field `RESERVED0` reader - 12:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY` reader - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BoundaryR = crate::FieldReader;
#[doc = "Field `BOUNDARY` writer - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BOUNDARY_MSB` reader - 20:20\\]
Non-Secure boundary address MSB HW controlled."]
pub type BoundaryMsbR = crate::BitReader;
#[doc = "Field `RESERVED21` reader - 30:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader<u16>;
#[doc = "Field `PARITY` reader - 31:31\\]
Register parity bit"]
pub type ParityR = crate::BitReader;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:19 - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn boundary(&self) -> BoundaryR {
        BoundaryR::new(((self.bits >> 13) & 0x7f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Non-Secure boundary address MSB HW controlled."]
    #[inline(always)]
    pub fn boundary_msb(&self) -> BoundaryMsbR {
        BoundaryMsbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:30 - 30:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 13:19 - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn boundary(&mut self) -> BoundaryW<NvmnsaddrSpec> {
        BoundaryW::new(self, 13)
    }
}
#[doc = "NVM Non-Secure boundary Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvmnsaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvmnsaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvmnsaddrSpec;
impl crate::RegisterSpec for NvmnsaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvmnsaddr::R`](R) reader structure"]
impl crate::Readable for NvmnsaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`nvmnsaddr::W`](W) writer structure"]
impl crate::Writable for NvmnsaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVMNSADDR to value 0x8010_0000"]
impl crate::Resettable for NvmnsaddrSpec {
    const RESET_VALUE: u32 = 0x8010_0000;
}
