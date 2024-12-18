#[doc = "Register `DOUT23_20` reader"]
pub type R = crate::R<Dout23_20Spec>;
#[doc = "Register `DOUT23_20` writer"]
pub type W = crate::W<Dout23_20Spec>;
#[doc = "Field `DIO20` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#20, if the corresponding DOE47_0 bitfield is set."]
pub type Dio20R = crate::BitReader;
#[doc = "Field `DIO20` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#20, if the corresponding DOE47_0 bitfield is set."]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DIO21` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#21, if the corresponding DOE47_0 bitfield is set."]
pub type Dio21R = crate::BitReader;
#[doc = "Field `DIO21` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#21, if the corresponding DOE47_0 bitfield is set."]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `DIO22` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#22, if the corresponding DOE47_0 bitfield is set."]
pub type Dio22R = crate::BitReader;
#[doc = "Field `DIO22` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#22, if the corresponding DOE47_0 bitfield is set."]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `DIO23` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#23, if the corresponding DOE47_0 bitfield is set."]
pub type Dio23R = crate::BitReader;
#[doc = "Field `DIO23` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#23, if the corresponding DOE47_0 bitfield is set."]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#20, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#21, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#22, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#23, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#20, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio20(&mut self) -> Dio20W<Dout23_20Spec> {
        Dio20W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#21, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio21(&mut self) -> Dio21W<Dout23_20Spec> {
        Dio21W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#22, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio22(&mut self) -> Dio22W<Dout23_20Spec> {
        Dio22W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#23, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio23(&mut self) -> Dio23W<Dout23_20Spec> {
        Dio23W::new(self, 24)
    }
}
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout23_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout23_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout23_20Spec;
impl crate::RegisterSpec for Dout23_20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout23_20::R`](R) reader structure"]
impl crate::Readable for Dout23_20Spec {}
#[doc = "`write(|w| ..)` method takes [`dout23_20::W`](W) writer structure"]
impl crate::Writable for Dout23_20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT23_20 to value 0"]
impl crate::Resettable for Dout23_20Spec {
    const RESET_VALUE: u32 = 0;
}
