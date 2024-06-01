#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DevidSpec>;
#[doc = "Register `DEVID` writer"]
pub type W = crate::W<DevidSpec>;
#[doc = "Field `DEVID` reader - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
pub type DevidR = crate::FieldReader<u32>;
#[doc = "Field `DEVID` writer - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
pub type DevidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[inline(always)]
    pub fn devid(&self) -> DevidR {
        DevidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[inline(always)]
    #[must_use]
    pub fn devid(&mut self) -> DevidW<DevidSpec> {
        DevidW::new(self, 0)
    }
}
#[doc = "Device ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevidSpec;
impl crate::RegisterSpec for DevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DevidSpec {}
#[doc = "`write(|w| ..)` method takes [`devid::W`](W) writer structure"]
impl crate::Writable for DevidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVID to value 0x0ca0"]
impl crate::Resettable for DevidSpec {
    const RESET_VALUE: u32 = 0x0ca0;
}
