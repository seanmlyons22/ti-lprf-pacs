#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BASEPTR` reader - 31:8\\]
This register point to the base address for the primary data structures of each uDMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when uDMA is in usage"]
pub type BaseptrR = crate::FieldReader<u32>;
#[doc = "Field `BASEPTR` writer - 31:8\\]
This register point to the base address for the primary data structures of each uDMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when uDMA is in usage"]
pub type BaseptrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
This register point to the base address for the primary data structures of each uDMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when uDMA is in usage"]
    #[inline(always)]
    pub fn baseptr(&self) -> BaseptrR {
        BaseptrR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<CtrlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
This register point to the base address for the primary data structures of each uDMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when uDMA is in usage"]
    #[inline(always)]
    #[must_use]
    pub fn baseptr(&mut self) -> BaseptrW<CtrlSpec> {
        BaseptrW::new(self, 8)
    }
}
#[doc = "Channel Control Data Base Pointer Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
