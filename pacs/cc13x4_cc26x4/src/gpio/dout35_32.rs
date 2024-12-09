#[doc = "Register `DOUT35_32` reader"]
pub type R = crate::R<Dout35_32Spec>;
#[doc = "Register `DOUT35_32` writer"]
pub type W = crate::W<Dout35_32Spec>;
#[doc = "Field `DIO32` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#32, if the corresponding DOE47_0 bitfield is set."]
pub type Dio32R = crate::BitReader;
#[doc = "Field `DIO32` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#32, if the corresponding DOE47_0 bitfield is set."]
pub type Dio32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DIO33` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#33, if the corresponding DOE47_0 bitfield is set."]
pub type Dio33R = crate::BitReader;
#[doc = "Field `DIO33` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#33, if the corresponding DOE47_0 bitfield is set."]
pub type Dio33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `DIO34` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#34, if the corresponding DOE47_0 bitfield is set."]
pub type Dio34R = crate::BitReader;
#[doc = "Field `DIO34` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#34, if the corresponding DOE47_0 bitfield is set."]
pub type Dio34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `DIO35` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#35, if the corresponding DOE47_0 bitfield is set."]
pub type Dio35R = crate::BitReader;
#[doc = "Field `DIO35` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#35, if the corresponding DOE47_0 bitfield is set."]
pub type Dio35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#32, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio32(&self) -> Dio32R {
        Dio32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#33, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio33(&self) -> Dio33R {
        Dio33R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#34, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio34(&self) -> Dio34R {
        Dio34R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#35, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio35(&self) -> Dio35R {
        Dio35R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#32, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio32(&mut self) -> Dio32W<Dout35_32Spec> {
        Dio32W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#33, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio33(&mut self) -> Dio33W<Dout35_32Spec> {
        Dio33W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#34, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio34(&mut self) -> Dio34W<Dout35_32Spec> {
        Dio34W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#35, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio35(&mut self) -> Dio35W<Dout35_32Spec> {
        Dio35W::new(self, 24)
    }
}
#[doc = "Data Out 35 to 32 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout35_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout35_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout35_32Spec;
impl crate::RegisterSpec for Dout35_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout35_32::R`](R) reader structure"]
impl crate::Readable for Dout35_32Spec {}
#[doc = "`write(|w| ..)` method takes [`dout35_32::W`](W) writer structure"]
impl crate::Writable for Dout35_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT35_32 to value 0"]
impl crate::Resettable for Dout35_32Spec {
    const RESET_VALUE: u32 = 0;
}
