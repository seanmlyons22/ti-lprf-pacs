#[doc = "Register `NVMNSCADDR` reader"]
pub type R = crate::R<NvmnscaddrSpec>;
#[doc = "Register `NVMNSCADDR` writer"]
pub type W = crate::W<NvmnscaddrSpec>;
#[doc = "Field `RESERVED0` reader - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY` reader - 19:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BoundaryR = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY` writer - 19:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RESERVED20` reader - 30:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `PARITY` reader - 31:31\\]
Register parity bit."]
pub type ParityR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn boundary(&self) -> BoundaryR {
        BoundaryR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:30 - 30:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:19 - 19:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn boundary(&mut self) -> BoundaryW<NvmnscaddrSpec> {
        BoundaryW::new(self, 10)
    }
}
#[doc = "NVM Non-Secure Callable boundary Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvmnscaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvmnscaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvmnscaddrSpec;
impl crate::RegisterSpec for NvmnscaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvmnscaddr::R`](R) reader structure"]
impl crate::Readable for NvmnscaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`nvmnscaddr::W`](W) writer structure"]
impl crate::Writable for NvmnscaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVMNSCADDR to value 0"]
impl crate::Resettable for NvmnscaddrSpec {
    const RESET_VALUE: u32 = 0;
}
