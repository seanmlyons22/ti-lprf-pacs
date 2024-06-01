#[doc = "Register `MAC_15_4_0` reader"]
pub type R = crate::R<Mac15_4_0Spec>;
#[doc = "Register `MAC_15_4_0` writer"]
pub type W = crate::W<Mac15_4_0Spec>;
#[doc = "Field `ADDR_0_31` reader - 31:0\\]
The first 32-bits of the 64-bit MAC 15.4 address"]
pub type Addr0_31R = crate::FieldReader<u32>;
#[doc = "Field `ADDR_0_31` writer - 31:0\\]
The first 32-bits of the 64-bit MAC 15.4 address"]
pub type Addr0_31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The first 32-bits of the 64-bit MAC 15.4 address"]
    #[inline(always)]
    pub fn addr_0_31(&self) -> Addr0_31R {
        Addr0_31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The first 32-bits of the 64-bit MAC 15.4 address"]
    #[inline(always)]
    #[must_use]
    pub fn addr_0_31(&mut self) -> Addr0_31W<Mac15_4_0Spec> {
        Addr0_31W::new(self, 0)
    }
}
#[doc = "MAC IEEE 802.15.4 Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_15_4_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_15_4_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mac15_4_0Spec;
impl crate::RegisterSpec for Mac15_4_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_15_4_0::R`](R) reader structure"]
impl crate::Readable for Mac15_4_0Spec {}
#[doc = "`write(|w| ..)` method takes [`mac_15_4_0::W`](W) writer structure"]
impl crate::Writable for Mac15_4_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_15_4_0 to value 0"]
impl crate::Resettable for Mac15_4_0Spec {
    const RESET_VALUE: u32 = 0;
}
