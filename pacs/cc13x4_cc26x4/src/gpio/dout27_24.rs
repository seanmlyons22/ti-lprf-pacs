#[doc = "Register `DOUT27_24` reader"]
pub type R = crate::R<Dout27_24Spec>;
#[doc = "Register `DOUT27_24` writer"]
pub type W = crate::W<Dout27_24Spec>;
#[doc = "Field `DIO24` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#24, if the corresponding DOE47_0 bitfield is set."]
pub type Dio24R = crate::BitReader;
#[doc = "Field `DIO24` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#24, if the corresponding DOE47_0 bitfield is set."]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DIO25` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#25, if the corresponding DOE47_0 bitfield is set."]
pub type Dio25R = crate::BitReader;
#[doc = "Field `DIO25` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#25, if the corresponding DOE47_0 bitfield is set."]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `DIO26` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#26, if the corresponding DOE47_0 bitfield is set."]
pub type Dio26R = crate::BitReader;
#[doc = "Field `DIO26` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#26, if the corresponding DOE47_0 bitfield is set."]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `DIO27` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#27, if the corresponding DOE47_0 bitfield is set."]
pub type Dio27R = crate::BitReader;
#[doc = "Field `DIO27` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#27, if the corresponding DOE47_0 bitfield is set."]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#24, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#25, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#26, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#27, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#24, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio24(&mut self) -> Dio24W<Dout27_24Spec> {
        Dio24W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#25, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio25(&mut self) -> Dio25W<Dout27_24Spec> {
        Dio25W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#26, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio26(&mut self) -> Dio26W<Dout27_24Spec> {
        Dio26W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#27, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio27(&mut self) -> Dio27W<Dout27_24Spec> {
        Dio27W::new(self, 24)
    }
}
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout27_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout27_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout27_24Spec;
impl crate::RegisterSpec for Dout27_24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout27_24::R`](R) reader structure"]
impl crate::Readable for Dout27_24Spec {}
#[doc = "`write(|w| ..)` method takes [`dout27_24::W`](W) writer structure"]
impl crate::Writable for Dout27_24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT27_24 to value 0"]
impl crate::Resettable for Dout27_24Spec {
    const RESET_VALUE: u32 = 0;
}
