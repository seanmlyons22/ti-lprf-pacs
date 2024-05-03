#[doc = "Register `IEEE_MAC_0` reader"]
pub type R = crate::R<IeeeMac0Spec>;
#[doc = "Register `IEEE_MAC_0` writer"]
pub type W = crate::W<IeeeMac0Spec>;
#[doc = "Field `ADDR` reader - 31:0\\]
Bits\\[31:0\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Bits\\[31:0\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[31:0\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[31:0\\]
of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<IeeeMac0Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "IEEE MAC Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_mac_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_mac_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeeeMac0Spec;
impl crate::RegisterSpec for IeeeMac0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieee_mac_0::R`](R) reader structure"]
impl crate::Readable for IeeeMac0Spec {}
#[doc = "`write(|w| ..)` method takes [`ieee_mac_0::W`](W) writer structure"]
impl crate::Writable for IeeeMac0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEEE_MAC_0 to value 0xffff_ffff"]
impl crate::Resettable for IeeeMac0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
