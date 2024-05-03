#[doc = "Register `CLAIMMASK` reader"]
pub type R = crate::R<ClaimmaskSpec>;
#[doc = "Register `CLAIMMASK` writer"]
pub type W = crate::W<ClaimmaskSpec>;
#[doc = "Field `CLAIMMASK` reader - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
pub type ClaimmaskR = crate::FieldReader<u32>;
#[doc = "Field `CLAIMMASK` writer - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
pub type ClaimmaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[inline(always)]
    pub fn claimmask(&self) -> ClaimmaskR {
        ClaimmaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[inline(always)]
    #[must_use]
    pub fn claimmask(&mut self) -> ClaimmaskW<ClaimmaskSpec> {
        ClaimmaskW::new(self, 0)
    }
}
#[doc = "Claim Tag Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimmaskSpec;
impl crate::RegisterSpec for ClaimmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimmask::R`](R) reader structure"]
impl crate::Readable for ClaimmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`claimmask::W`](W) writer structure"]
impl crate::Writable for ClaimmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLAIMMASK to value 0x0f"]
impl crate::Resettable for ClaimmaskSpec {
    const RESET_VALUE: u32 = 0x0f;
}
