#[doc = "Register `AIFWCLKSRC` reader"]
pub type R = crate::R<AifwclksrcSpec>;
#[doc = "Register `AIFWCLKSRC` writer"]
pub type W = crate::W<AifwclksrcSpec>;
#[doc = "1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WclkSrc {
    #[doc = "2: Internal WCLK generator, from module PRCM"]
    Int = 2,
    #[doc = "1: External WCLK generator, from pad"]
    Ext = 1,
    #[doc = "0: None ('0')"]
    None = 0,
}
impl From<WclkSrc> for u8 {
    #[inline(always)]
    fn from(variant: WclkSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WclkSrc {
    type Ux = u8;
}
impl crate::IsEnum for WclkSrc {}
#[doc = "Field `WCLK_SRC` reader - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
pub type WclkSrcR = crate::FieldReader<WclkSrc>;
impl WclkSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WclkSrc {
        match self.bits {
            2 => WclkSrc::Int,
            1 => WclkSrc::Ext,
            0 => WclkSrc::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal WCLK generator, from module PRCM"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WclkSrc::Int
    }
    #[doc = "External WCLK generator, from pad"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == WclkSrc::Ext
    }
    #[doc = "None ('0')"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WclkSrc::None
    }
}
#[doc = "Field `WCLK_SRC` writer - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
pub type WclkSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, WclkSrc>;
impl<'a, REG> WclkSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal WCLK generator, from module PRCM"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(WclkSrc::Int)
    }
    #[doc = "External WCLK generator, from pad"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(WclkSrc::Ext)
    }
    #[doc = "None ('0')"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(WclkSrc::None)
    }
}
#[doc = "Field `WCLK_INV` reader - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
pub type WclkInvR = crate::BitReader;
#[doc = "Field `WCLK_INV` writer - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
pub type WclkInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline(always)]
    pub fn wclk_src(&self) -> WclkSrcR {
        WclkSrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline(always)]
    pub fn wclk_inv(&self) -> WclkInvR {
        WclkInvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_src(&mut self) -> WclkSrcW<AifwclksrcSpec> {
        WclkSrcW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_inv(&mut self) -> WclkInvW<AifwclksrcSpec> {
        WclkInvW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<AifwclksrcSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "WCLK Source Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifwclksrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifwclksrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AifwclksrcSpec;
impl crate::RegisterSpec for AifwclksrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aifwclksrc::R`](R) reader structure"]
impl crate::Readable for AifwclksrcSpec {}
#[doc = "`write(|w| ..)` method takes [`aifwclksrc::W`](W) writer structure"]
impl crate::Writable for AifwclksrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFWCLKSRC to value 0"]
impl crate::Resettable for AifwclksrcSpec {
    const RESET_VALUE: u32 = 0;
}
