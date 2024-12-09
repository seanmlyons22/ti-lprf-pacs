#[doc = "Register `DOUT31_28` reader"]
pub type R = crate::R<Dout31_28Spec>;
#[doc = "Register `DOUT31_28` writer"]
pub type W = crate::W<Dout31_28Spec>;
#[doc = "Field `DIO28` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE47_0 bitfield is set."]
pub type Dio28R = crate::BitReader;
#[doc = "Field `DIO28` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE47_0 bitfield is set."]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DIO29` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE47_0 bitfield is set."]
pub type Dio29R = crate::BitReader;
#[doc = "Field `DIO29` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE47_0 bitfield is set."]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `DIO30` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE47_0 bitfield is set."]
pub type Dio30R = crate::BitReader;
#[doc = "Field `DIO30` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE47_0 bitfield is set."]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `DIO31` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE47_0 bitfield is set."]
pub type Dio31R = crate::BitReader;
#[doc = "Field `DIO31` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE47_0 bitfield is set."]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio28(&mut self) -> Dio28W<Dout31_28Spec> {
        Dio28W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio29(&mut self) -> Dio29W<Dout31_28Spec> {
        Dio29W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio30(&mut self) -> Dio30W<Dout31_28Spec> {
        Dio30W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio31(&mut self) -> Dio31W<Dout31_28Spec> {
        Dio31W::new(self, 24)
    }
}
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout31_28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout31_28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout31_28Spec;
impl crate::RegisterSpec for Dout31_28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout31_28::R`](R) reader structure"]
impl crate::Readable for Dout31_28Spec {}
#[doc = "`write(|w| ..)` method takes [`dout31_28::W`](W) writer structure"]
impl crate::Writable for Dout31_28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT31_28 to value 0"]
impl crate::Resettable for Dout31_28Spec {
    const RESET_VALUE: u32 = 0;
}
