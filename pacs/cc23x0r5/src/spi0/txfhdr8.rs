#[doc = "Register `TXFHDR8` reader"]
pub type R = crate::R<Txfhdr8Spec>;
#[doc = "Register `TXFHDR8` writer"]
pub type W = crate::W<Txfhdr8Spec>;
#[doc = "31:0\\]
This field can be used to write one byte of header data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Data {
    #[doc = "1: Highest possible value"]
    Maximum = 1,
    #[doc = "0: Smallest possible value"]
    Minimum = 0,
}
impl From<Data> for u32 {
    #[inline(always)]
    fn from(variant: Data) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Data {
    type Ux = u32;
}
impl crate::IsEnum for Data {}
#[doc = "Field `DATA` reader - 31:0\\]
This field can be used to write one byte of header data"]
pub type DataR = crate::FieldReader<Data>;
impl DataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Data> {
        match self.bits {
            1 => Some(Data::Maximum),
            0 => Some(Data::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Data::Maximum
    }
    #[doc = "Smallest possible value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Data::Minimum
    }
}
#[doc = "Field `DATA` writer - 31:0\\]
This field can be used to write one byte of header data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, Data>;
impl<'a, REG> DataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Data::Maximum)
    }
    #[doc = "Smallest possible value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Data::Minimum)
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field can be used to write one byte of header data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field can be used to write one byte of header data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Txfhdr8Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Header update reigster for 8 bits of header data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txfhdr8Spec;
impl crate::RegisterSpec for Txfhdr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfhdr8::R`](R) reader structure"]
impl crate::Readable for Txfhdr8Spec {}
#[doc = "`write(|w| ..)` method takes [`txfhdr8::W`](W) writer structure"]
impl crate::Writable for Txfhdr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFHDR8 to value 0"]
impl crate::Resettable for Txfhdr8Spec {
    const RESET_VALUE: u32 = 0;
}