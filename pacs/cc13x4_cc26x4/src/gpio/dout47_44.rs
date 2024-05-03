#[doc = "Register `DOUT47_44` reader"]
pub type R = crate::R<Dout47_44Spec>;
#[doc = "Register `DOUT47_44` writer"]
pub type W = crate::W<Dout47_44Spec>;
#[doc = "Field `DIO44` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#44, if the corresponding DOE47_0 bitfield is set."]
pub type Dio44R = crate::BitReader;
#[doc = "Field `DIO44` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#44, if the corresponding DOE47_0 bitfield is set."]
pub type Dio44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO45` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#45, if the corresponding DOE47_0 bitfield is set."]
pub type Dio45R = crate::BitReader;
#[doc = "Field `DIO45` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#45, if the corresponding DOE47_0 bitfield is set."]
pub type Dio45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO46` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#46, if the corresponding DOE47_0 bitfield is set."]
pub type Dio46R = crate::BitReader;
#[doc = "Field `DIO46` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#46, if the corresponding DOE47_0 bitfield is set."]
pub type Dio46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `RESERVED17` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO47` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#47, if the corresponding DOE47_0 bitfield is set."]
pub type Dio47R = crate::BitReader;
#[doc = "Field `DIO47` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#47, if the corresponding DOE47_0 bitfield is set."]
pub type Dio47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#44, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio44(&self) -> Dio44R {
        Dio44R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#45, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio45(&self) -> Dio45R {
        Dio45R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#46, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio46(&self) -> Dio46R {
        Dio46R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#47, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio47(&self) -> Dio47R {
        Dio47R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#44, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio44(&mut self) -> Dio44W<Dout47_44Spec> {
        Dio44W::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Dout47_44Spec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#45, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio45(&mut self) -> Dio45W<Dout47_44Spec> {
        Dio45W::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Dout47_44Spec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#46, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio46(&mut self) -> Dio46W<Dout47_44Spec> {
        Dio46W::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<Dout47_44Spec> {
        Reserved17W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#47, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio47(&mut self) -> Dio47W<Dout47_44Spec> {
        Dio47W::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Dout47_44Spec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "Data Out 47 to 44 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout47_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout47_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout47_44Spec;
impl crate::RegisterSpec for Dout47_44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout47_44::R`](R) reader structure"]
impl crate::Readable for Dout47_44Spec {}
#[doc = "`write(|w| ..)` method takes [`dout47_44::W`](W) writer structure"]
impl crate::Writable for Dout47_44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT47_44 to value 0"]
impl crate::Resettable for Dout47_44Spec {
    const RESET_VALUE: u32 = 0;
}
