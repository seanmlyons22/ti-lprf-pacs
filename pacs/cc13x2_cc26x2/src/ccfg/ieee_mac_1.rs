#[doc = "Register `IEEE_MAC_1` reader"]
pub type R = crate::R<IeeeMac1Spec>;
#[doc = "Register `IEEE_MAC_1` writer"]
pub type W = crate::W<IeeeMac1Spec>;
#[doc = "Field `ADDR` reader - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<IeeeMac1Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "IEEE MAC Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_mac_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_mac_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeeeMac1Spec;
impl crate::RegisterSpec for IeeeMac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieee_mac_1::R`](R) reader structure"]
impl crate::Readable for IeeeMac1Spec {}
#[doc = "`write(|w| ..)` method takes [`ieee_mac_1::W`](W) writer structure"]
impl crate::Writable for IeeeMac1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEEE_MAC_1 to value 0xffff_ffff"]
impl crate::Resettable for IeeeMac1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
