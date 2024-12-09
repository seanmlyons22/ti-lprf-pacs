#[doc = "Register `RXCRC` reader"]
pub type R = crate::R<RxcrcSpec>;
#[doc = "Register `RXCRC` writer"]
pub type W = crate::W<RxcrcSpec>;
#[doc = "Field `DATA` reader - 15:0\\]
CRC value"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 15:0\\]
CRC value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
CRC value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
CRC value"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<RxcrcSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Receive CRC register. Reading this register provides the computed CRC value from the receive side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcrcSpec;
impl crate::RegisterSpec for RxcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrc::R`](R) reader structure"]
impl crate::Readable for RxcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`rxcrc::W`](W) writer structure"]
impl crate::Writable for RxcrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RxcrcSpec {
    const RESET_VALUE: u32 = 0;
}
