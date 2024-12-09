#[doc = "Register `FFSR` reader"]
pub type R = crate::R<FfsrSpec>;
#[doc = "Register `FFSR` writer"]
pub type W = crate::W<FfsrSpec>;
#[doc = "Field `RESERVED0` reader - 2:0\\]
This field always reads as zero"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `FTNONSTOP` reader - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
pub type FtnonstopR = crate::BitReader;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This field always reads as zero"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
    #[inline(always)]
    pub fn ftnonstop(&self) -> FtnonstopR {
        FtnonstopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {}
#[doc = "Formatter and Flush Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfsrSpec;
impl crate::RegisterSpec for FfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffsr::R`](R) reader structure"]
impl crate::Readable for FfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffsr::W`](W) writer structure"]
impl crate::Writable for FfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFSR to value 0x08"]
impl crate::Resettable for FfsrSpec {
    const RESET_VALUE: u32 = 0x08;
}
