#[doc = "Register `KEYSIZE` reader"]
pub type R = crate::R<KeysizeSpec>;
#[doc = "Register `KEYSIZE` writer"]
pub type W = crate::W<KeysizeSpec>;
#[doc = "1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "3: Not supported"]
    _256Bit = 3,
    #[doc = "2: Not supported"]
    _192Bit = 2,
    #[doc = "1: 128 bits"]
    _128Bit = 1,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            3 => Some(Size::_256Bit),
            2 => Some(Size::_192Bit),
            1 => Some(Size::_128Bit),
            _ => None,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_256_bit(&self) -> bool {
        *self == Size::_256Bit
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_192_bit(&self) -> bool {
        *self == Size::_192Bit
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn is_128_bit(&self) -> bool {
        *self == Size::_128Bit
    }
}
#[doc = "Field `SIZE` writer - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn _256_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_256Bit)
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn _192_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_192Bit)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _128_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_128Bit)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<KeysizeSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<KeysizeSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Key Size This register defines the size of the keys that are written with DMA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keysize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keysize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeysizeSpec;
impl crate::RegisterSpec for KeysizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keysize::R`](R) reader structure"]
impl crate::Readable for KeysizeSpec {}
#[doc = "`write(|w| ..)` method takes [`keysize::W`](W) writer structure"]
impl crate::Writable for KeysizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYSIZE to value 0x01"]
impl crate::Resettable for KeysizeSpec {
    const RESET_VALUE: u32 = 0x01;
}
