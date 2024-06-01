#[doc = "Register `CONFIG_IF_ADC` reader"]
pub type R = crate::R<ConfigIfAdcSpec>;
#[doc = "Register `CONFIG_IF_ADC` writer"]
pub type W = crate::W<ConfigIfAdcSpec>;
#[doc = "Field `IFANALDO_TRIM_OUTPUT` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type IfanaldoTrimOutputR = crate::FieldReader;
#[doc = "Field `IFANALDO_TRIM_OUTPUT` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type IfanaldoTrimOutputW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IFDIGLDO_TRIM_OUTPUT` reader - 9:5\\]
Internal. Only to be used through TI provided API."]
pub type IfdigldoTrimOutputR = crate::FieldReader;
#[doc = "Field `IFDIGLDO_TRIM_OUTPUT` writer - 9:5\\]
Internal. Only to be used through TI provided API."]
pub type IfdigldoTrimOutputW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INT2ADJ` reader - 13:10\\]
Internal. Only to be used through TI provided API."]
pub type Int2adjR = crate::FieldReader;
#[doc = "Field `INT2ADJ` writer - 13:10\\]
Internal. Only to be used through TI provided API."]
pub type Int2adjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AAFCAP` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type AafcapR = crate::FieldReader;
#[doc = "Field `AAFCAP` writer - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type AafcapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FF1ADJ` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type Ff1adjR = crate::FieldReader;
#[doc = "Field `FF1ADJ` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type Ff1adjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INT3ADJ` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Int3adjR = crate::FieldReader;
#[doc = "Field `INT3ADJ` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Int3adjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FF3ADJ` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Ff3adjR = crate::FieldReader;
#[doc = "Field `FF3ADJ` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Ff3adjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FF2ADJ` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Ff2adjR = crate::FieldReader;
#[doc = "Field `FF2ADJ` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Ff2adjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifanaldo_trim_output(&self) -> IfanaldoTrimOutputR {
        IfanaldoTrimOutputR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifdigldo_trim_output(&self) -> IfdigldoTrimOutputR {
        IfdigldoTrimOutputR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int2adj(&self) -> Int2adjR {
        Int2adjR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aafcap(&self) -> AafcapR {
        AafcapR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff1adj(&self) -> Ff1adjR {
        Ff1adjR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int3adj(&self) -> Int3adjR {
        Int3adjR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff3adj(&self) -> Ff3adjR {
        Ff3adjR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff2adj(&self) -> Ff2adjR {
        Ff2adjR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ifanaldo_trim_output(&mut self) -> IfanaldoTrimOutputW<ConfigIfAdcSpec> {
        IfanaldoTrimOutputW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ifdigldo_trim_output(&mut self) -> IfdigldoTrimOutputW<ConfigIfAdcSpec> {
        IfdigldoTrimOutputW::new(self, 5)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn int2adj(&mut self) -> Int2adjW<ConfigIfAdcSpec> {
        Int2adjW::new(self, 10)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn aafcap(&mut self) -> AafcapW<ConfigIfAdcSpec> {
        AafcapW::new(self, 14)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ff1adj(&mut self) -> Ff1adjW<ConfigIfAdcSpec> {
        Ff1adjW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn int3adj(&mut self) -> Int3adjW<ConfigIfAdcSpec> {
        Int3adjW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ff3adj(&mut self) -> Ff3adjW<ConfigIfAdcSpec> {
        Ff3adjW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ff2adj(&mut self) -> Ff2adjW<ConfigIfAdcSpec> {
        Ff2adjW::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_if_adc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_if_adc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigIfAdcSpec;
impl crate::RegisterSpec for ConfigIfAdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_if_adc::R`](R) reader structure"]
impl crate::Readable for ConfigIfAdcSpec {}
#[doc = "`write(|w| ..)` method takes [`config_if_adc::W`](W) writer structure"]
impl crate::Writable for ConfigIfAdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_IF_ADC to value 0x3460_f400"]
impl crate::Resettable for ConfigIfAdcSpec {
    const RESET_VALUE: u32 = 0x3460_f400;
}
