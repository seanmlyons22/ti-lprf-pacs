#[doc = "Register `TEST1` reader"]
pub type R = crate::R<Test1Spec>;
#[doc = "Register `TEST1` writer"]
pub type W = crate::W<Test1Spec>;
#[doc = "Field `DTB_MUXSEL` reader - 4:0\\]
DTB mux select."]
pub type DtbMuxselR = crate::FieldReader;
#[doc = "Field `DTB_MUXSEL` writer - 4:0\\]
DTB mux select."]
pub type DtbMuxselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DTB mux select."]
    #[inline(always)]
    pub fn dtb_muxsel(&self) -> DtbMuxselR {
        DtbMuxselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DTB mux select."]
    #[inline(always)]
    #[must_use]
    pub fn dtb_muxsel(&mut self) -> DtbMuxselW<Test1Spec> {
        DtbMuxselW::new(self, 0)
    }
}
#[doc = "Test 1 register. This is used to select ADC internal signals on DTB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test1Spec;
impl crate::RegisterSpec for Test1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test1::R`](R) reader structure"]
impl crate::Readable for Test1Spec {}
#[doc = "`write(|w| ..)` method takes [`test1::W`](W) writer structure"]
impl crate::Writable for Test1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST1 to value 0"]
impl crate::Resettable for Test1Spec {
    const RESET_VALUE: u32 = 0;
}
