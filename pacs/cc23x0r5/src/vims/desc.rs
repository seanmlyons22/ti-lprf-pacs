#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Register `DESC` writer"]
pub type W = crate::W<DescSpec>;
#[doc = "Field `MINREV` reader - 3:0\\]
Minor revision of IP"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MINREV` writer - 3:0\\]
Minor revision of IP"]
pub type MinrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJREV` reader - 7:4\\]
Major revision of IP"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `MAJREV` writer - 7:4\\]
Major revision of IP"]
pub type MajrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INSTIDX` reader - 11:8\\]
Instance Index within the device. This will be a parameter to the RTL for modules that can have multiple instances."]
pub type InstidxR = crate::FieldReader;
#[doc = "Field `INSTIDX` writer - 11:8\\]
Instance Index within the device. This will be a parameter to the RTL for modules that can have multiple instances."]
pub type InstidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STDIPOFF` reader - 15:12\\]
Standard IP registers offset. Value 0 indicates Standard IP registers are not present. Any other value between 1 to 15 indicates standard IP registers start from address offset 64 * STDIPOFF from base address."]
pub type StdipoffR = crate::FieldReader;
#[doc = "Field `STDIPOFF` writer - 15:12\\]
Standard IP registers offset. Value 0 indicates Standard IP registers are not present. Any other value between 1 to 15 indicates standard IP registers start from address offset 64 * STDIPOFF from base address."]
pub type StdipoffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODID` reader - 31:16\\]
Module identification contains a unique peripheral identification number."]
pub type ModidR = crate::FieldReader<u16>;
#[doc = "Field `MODID` writer - 31:16\\]
Module identification contains a unique peripheral identification number."]
pub type ModidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of IP"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of IP"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Instance Index within the device. This will be a parameter to the RTL for modules that can have multiple instances."]
    #[inline(always)]
    pub fn instidx(&self) -> InstidxR {
        InstidxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Standard IP registers offset. Value 0 indicates Standard IP registers are not present. Any other value between 1 to 15 indicates standard IP registers start from address offset 64 * STDIPOFF from base address."]
    #[inline(always)]
    pub fn stdipoff(&self) -> StdipoffR {
        StdipoffR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identification contains a unique peripheral identification number."]
    #[inline(always)]
    pub fn modid(&self) -> ModidR {
        ModidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of IP"]
    #[inline(always)]
    #[must_use]
    pub fn minrev(&mut self) -> MinrevW<DescSpec> {
        MinrevW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of IP"]
    #[inline(always)]
    #[must_use]
    pub fn majrev(&mut self) -> MajrevW<DescSpec> {
        MajrevW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Instance Index within the device. This will be a parameter to the RTL for modules that can have multiple instances."]
    #[inline(always)]
    #[must_use]
    pub fn instidx(&mut self) -> InstidxW<DescSpec> {
        InstidxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Standard IP registers offset. Value 0 indicates Standard IP registers are not present. Any other value between 1 to 15 indicates standard IP registers start from address offset 64 * STDIPOFF from base address."]
    #[inline(always)]
    #[must_use]
    pub fn stdipoff(&mut self) -> StdipoffW<DescSpec> {
        StdipoffW::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identification contains a unique peripheral identification number."]
    #[inline(always)]
    #[must_use]
    pub fn modid(&mut self) -> ModidW<DescSpec> {
        ModidW::new(self, 16)
    }
}
#[doc = "This register identifies the peripheral and its exact version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescSpec;
impl crate::RegisterSpec for DescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`desc::R`](R) reader structure"]
impl crate::Readable for DescSpec {}
#[doc = "`write(|w| ..)` method takes [`desc::W`](W) writer structure"]
impl crate::Writable for DescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESC to value 0xd140_0010"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0xd140_0010;
}
