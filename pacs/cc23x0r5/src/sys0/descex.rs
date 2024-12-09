#[doc = "Register `DESCEX` reader"]
pub type R = crate::R<DescexSpec>;
#[doc = "Register `DESCEX` writer"]
pub type W = crate::W<DescexSpec>;
#[doc = "Field `TBITS` reader - 15:0\\]
Total number of trim bits"]
pub type TbitsR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` reader - 31:16\\]
Reserved"]
pub type Reserved20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Total number of trim bits"]
    #[inline(always)]
    pub fn tbits(&self) -> TbitsR {
        TbitsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Extended Description Register. This register provides configuration details of the IP to software drivers and end users.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescexSpec;
impl crate::RegisterSpec for DescexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex::R`](R) reader structure"]
impl crate::Readable for DescexSpec {}
#[doc = "`write(|w| ..)` method takes [`descex::W`](W) writer structure"]
impl crate::Writable for DescexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX to value 0x0129"]
impl crate::Resettable for DescexSpec {
    const RESET_VALUE: u32 = 0x0129;
}
