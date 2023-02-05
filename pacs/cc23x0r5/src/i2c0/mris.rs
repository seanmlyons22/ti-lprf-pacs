#[doc = "Register `MRIS` reader"]
pub type R = crate::R<MrisSpec>;
#[doc = "Register `MRIS` writer"]
pub type W = crate::W<MrisSpec>;
#[doc = "Field `RIS` reader - 0:0\\]
Raw interrupt status 0 - Interrupt did not occur 1 - Interrupt occured This bit is cleared by writing 1 to the MICR.IC bit."]
pub type RisR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt status 0 - Interrupt did not occur 1 - Interrupt occured This bit is cleared by writing 1 to the MICR.IC bit."]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrisSpec;
impl crate::RegisterSpec for MrisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mris::R`](R) reader structure"]
impl crate::Readable for MrisSpec {}
#[doc = "`write(|w| ..)` method takes [`mris::W`](W) writer structure"]
impl crate::Writable for MrisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRIS to value 0"]
impl crate::Resettable for MrisSpec {
    const RESET_VALUE: u32 = 0;
}
