#[doc = "Register `MISOCFG` reader"]
pub type R = crate::R<MisocfgSpec>;
#[doc = "Register `MISOCFG` writer"]
pub type W = crate::W<MisocfgSpec>;
#[doc = "Field `AUXIO` reader - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
pub type AuxioR = crate::FieldReader;
#[doc = "Field `AUXIO` writer - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
pub type AuxioW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
    #[inline(always)]
    pub fn auxio(&self) -> AuxioR {
        AuxioR::new((self.bits & 0x1f) as u8)
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
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
    #[inline(always)]
    #[must_use]
    pub fn auxio(&mut self) -> AuxioW<MisocfgSpec> {
        AuxioW::new(self, 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<MisocfgSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "MISO Configuration Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misocfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misocfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisocfgSpec;
impl crate::RegisterSpec for MisocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misocfg::R`](R) reader structure"]
impl crate::Readable for MisocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`misocfg::W`](W) writer structure"]
impl crate::Writable for MisocfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISOCFG to value 0"]
impl crate::Resettable for MisocfgSpec {
    const RESET_VALUE: u32 = 0;
}
