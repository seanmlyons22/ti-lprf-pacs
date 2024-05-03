#[doc = "Register `OPTIONS` reader"]
pub type R = crate::R<OptionsSpec>;
#[doc = "Register `OPTIONS` writer"]
pub type W = crate::W<OptionsSpec>;
#[doc = "Field `PLB_INTERFACE` reader - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
pub type PlbInterfaceR = crate::BitReader;
#[doc = "Field `PLB_INTERFACE` writer - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
pub type PlbInterfaceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_INTERFACE` reader - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
pub type AhbInterfaceR = crate::BitReader;
#[doc = "Field `AHB_INTERFACE` writer - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
pub type AhbInterfaceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_IS_ASYNC` reader - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
pub type AhbIsAsyncR = crate::BitReader;
#[doc = "Field `AHB_IS_ASYNC` writer - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
pub type AhbIsAsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_INTERFACE` reader - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
pub type AxiInterfaceR = crate::BitReader;
#[doc = "Field `AXI_INTERFACE` writer - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
pub type AxiInterfaceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 7:4\\]
Ignore on read"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Ignore on read"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EIP28_PRESENT` reader - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
pub type Eip28PresentR = crate::BitReader;
#[doc = "Field `EIP28_PRESENT` writer - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
pub type Eip28PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIP76_PRESENT` reader - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
pub type Eip76PresentR = crate::BitReader;
#[doc = "Field `EIP76_PRESENT` writer - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
pub type Eip76PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIC_PRESENT` reader - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
pub type AicPresentR = crate::BitReader;
#[doc = "Field `AIC_PRESENT` writer - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
pub type AicPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Ignore on read"]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Ignore on read"]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
    #[inline(always)]
    pub fn plb_interface(&self) -> PlbInterfaceR {
        PlbInterfaceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
    #[inline(always)]
    pub fn ahb_interface(&self) -> AhbInterfaceR {
        AhbInterfaceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
    #[inline(always)]
    pub fn ahb_is_async(&self) -> AhbIsAsyncR {
        AhbIsAsyncR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
    #[inline(always)]
    pub fn axi_interface(&self) -> AxiInterfaceR {
        AxiInterfaceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
    #[inline(always)]
    pub fn eip28_present(&self) -> Eip28PresentR {
        Eip28PresentR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
    #[inline(always)]
    pub fn eip76_present(&self) -> Eip76PresentR {
        Eip76PresentR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
    #[inline(always)]
    pub fn aic_present(&self) -> AicPresentR {
        AicPresentR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
    #[inline(always)]
    #[must_use]
    pub fn plb_interface(&mut self) -> PlbInterfaceW<OptionsSpec> {
        PlbInterfaceW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_interface(&mut self) -> AhbInterfaceW<OptionsSpec> {
        AhbInterfaceW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_is_async(&mut self) -> AhbIsAsyncW<OptionsSpec> {
        AhbIsAsyncW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
    #[inline(always)]
    #[must_use]
    pub fn axi_interface(&mut self) -> AxiInterfaceW<OptionsSpec> {
        AxiInterfaceW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<OptionsSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn eip28_present(&mut self) -> Eip28PresentW<OptionsSpec> {
        Eip28PresentW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn eip76_present(&mut self) -> Eip76PresentW<OptionsSpec> {
        Eip76PresentW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn aic_present(&mut self) -> AicPresentW<OptionsSpec> {
        AicPresentW::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<OptionsSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "PKA Options register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`options::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionsSpec;
impl crate::RegisterSpec for OptionsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`options::R`](R) reader structure"]
impl crate::Readable for OptionsSpec {}
#[doc = "`write(|w| ..)` method takes [`options::W`](W) writer structure"]
impl crate::Writable for OptionsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTIONS to value 0x0102"]
impl crate::Resettable for OptionsSpec {
    const RESET_VALUE: u32 = 0x0102;
}
