#[doc = "Register `PDSTAT1` reader"]
pub type R = crate::R<Pdstat1Spec>;
#[doc = "Register `PDSTAT1` writer"]
pub type W = crate::W<Pdstat1Spec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `CPU_ON` reader - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
pub type CpuOnR = crate::BitReader;
#[doc = "Field `RFC_ON` reader - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
pub type RfcOnR = crate::BitReader;
#[doc = "Field `VIMS_ON` reader - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
pub type VimsOnR = crate::BitReader;
#[doc = "Field `BUS_ON` reader - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
pub type BusOnR = crate::BitReader;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
    #[inline(always)]
    pub fn cpu_on(&self) -> CpuOnR {
        CpuOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RfcOnR {
        RfcOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
    #[inline(always)]
    pub fn vims_on(&self) -> VimsOnR {
        VimsOnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
    #[inline(always)]
    pub fn bus_on(&self) -> BusOnR {
        BusOnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {}
#[doc = "Power Manager Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdstat1Spec;
impl crate::RegisterSpec for Pdstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdstat1::R`](R) reader structure"]
impl crate::Readable for Pdstat1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdstat1::W`](W) writer structure"]
impl crate::Writable for Pdstat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSTAT1 to value 0x1a"]
impl crate::Resettable for Pdstat1Spec {
    const RESET_VALUE: u32 = 0x1a;
}
