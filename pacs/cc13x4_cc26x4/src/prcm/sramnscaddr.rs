#[doc = "Register `SRAMNSCADDR` reader"]
pub type R = crate::R<SramnscaddrSpec>;
#[doc = "Register `SRAMNSCADDR` writer"]
pub type W = crate::W<SramnscaddrSpec>;
#[doc = "Field `RESERVED0` reader - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED0` writer - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BOUNDARY` reader - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BoundaryR = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY` writer - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RESERVED19` reader - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED19` writer - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PARITY` reader - 31:31\\]
Register parity bit"]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - 31:31\\]
Register parity bit"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:18 - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn boundary(&self) -> BoundaryR {
        BoundaryR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 19:30 - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<SramnscaddrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 10:18 - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn boundary(&mut self) -> BoundaryW<SramnscaddrSpec> {
        BoundaryW::new(self, 10)
    }
    #[doc = "Bits 19:30 - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<SramnscaddrSpec> {
        Reserved19W::new(self, 19)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<SramnscaddrSpec> {
        ParityW::new(self, 31)
    }
}
#[doc = "SRAM Non-Secure Callable boundary Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sramnscaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sramnscaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramnscaddrSpec;
impl crate::RegisterSpec for SramnscaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramnscaddr::R`](R) reader structure"]
impl crate::Readable for SramnscaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sramnscaddr::W`](W) writer structure"]
impl crate::Writable for SramnscaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMNSCADDR to value 0"]
impl crate::Resettable for SramnscaddrSpec {
    const RESET_VALUE: u32 = 0;
}
