#[doc = "Register `DIVMSW` reader"]
pub type R = crate::R<DivmswSpec>;
#[doc = "Register `DIVMSW` writer"]
pub type W = crate::W<DivmswSpec>;
#[doc = "Field `MSW_ADDRESS` reader - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
pub type MswAddressR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED11` reader - 14:11\\]
Ignore on read"]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESULT_IS_ZERO` reader - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
pub type ResultIsZeroR = crate::BitReader;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Ignore on read"]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
    #[inline(always)]
    pub fn msw_address(&self) -> MswAddressR {
        MswAddressR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    pub fn result_is_zero(&self) -> ResultIsZeroR {
        ResultIsZeroR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when FUNCTION.RUN bit is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; this register provides information for the x-coordinate of the result point only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divmsw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divmsw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivmswSpec;
impl crate::RegisterSpec for DivmswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divmsw::R`](R) reader structure"]
impl crate::Readable for DivmswSpec {}
#[doc = "`write(|w| ..)` method takes [`divmsw::W`](W) writer structure"]
impl crate::Writable for DivmswSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVMSW to value 0x8000"]
impl crate::Resettable for DivmswSpec {
    const RESET_VALUE: u32 = 0x8000;
}
