#[doc = "Register `DOUT19_16` reader"]
pub type R = crate::R<Dout19_16Spec>;
#[doc = "Register `DOUT19_16` writer"]
pub type W = crate::W<Dout19_16Spec>;
#[doc = "Field `DIO16` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#16, if the corresponding DOE31_0 bitfield is set."]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DIO17` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#17, if the corresponding DOE31_0 bitfield is set."]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `DIO18` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#18, if the corresponding DOE31_0 bitfield is set."]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `DIO19` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#19, if the corresponding DOE31_0 bitfield is set."]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#16, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio16(&mut self) -> Dio16W<Dout19_16Spec> {
        Dio16W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#17, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio17(&mut self) -> Dio17W<Dout19_16Spec> {
        Dio17W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#18, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio18(&mut self) -> Dio18W<Dout19_16Spec> {
        Dio18W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#19, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio19(&mut self) -> Dio19W<Dout19_16Spec> {
        Dio19W::new(self, 24)
    }
}
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout19_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout19_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout19_16Spec;
impl crate::RegisterSpec for Dout19_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout19_16::R`](R) reader structure"]
impl crate::Readable for Dout19_16Spec {}
#[doc = "`write(|w| ..)` method takes [`dout19_16::W`](W) writer structure"]
impl crate::Writable for Dout19_16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT19_16 to value 0"]
impl crate::Resettable for Dout19_16Spec {
    const RESET_VALUE: u32 = 0;
}
