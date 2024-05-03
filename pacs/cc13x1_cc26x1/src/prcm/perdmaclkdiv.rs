#[doc = "Register `PERDMACLKDIV` reader"]
pub type R = crate::R<PerdmaclkdivSpec>;
#[doc = "Register `PERDMACLKDIV` writer"]
pub type W = crate::W<PerdmaclkdivSpec>;
#[doc = "3:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Div256 = 8,
    #[doc = "7: Internal. Only to be used through TI provided API."]
    Div128 = 7,
    #[doc = "6: Internal. Only to be used through TI provided API."]
    Div64 = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    Div32 = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Div16 = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    Div8 = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Div4 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Div2 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Div1 = 0,
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ratio {
    type Ux = u8;
}
impl crate::IsEnum for Ratio {}
#[doc = "Field `RATIO` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ratio> {
        match self.bits {
            8 => Some(Ratio::Div256),
            7 => Some(Ratio::Div128),
            6 => Some(Ratio::Div64),
            5 => Some(Ratio::Div32),
            4 => Some(Ratio::Div16),
            3 => Some(Ratio::Div8),
            2 => Some(Ratio::Div4),
            1 => Some(Ratio::Div2),
            0 => Some(Ratio::Div1),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Ratio::Div256
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Ratio::Div128
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Ratio::Div64
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Ratio::Div32
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Ratio::Div16
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ratio::Div8
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ratio::Div4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ratio::Div2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ratio::Div1
    }
}
#[doc = "Field `RATIO` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ratio>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div256)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div128)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div64)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div32)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div16)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div8)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div1)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<PerdmaclkdivSpec> {
        RatioW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<PerdmaclkdivSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perdmaclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perdmaclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerdmaclkdivSpec;
impl crate::RegisterSpec for PerdmaclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perdmaclkdiv::R`](R) reader structure"]
impl crate::Readable for PerdmaclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`perdmaclkdiv::W`](W) writer structure"]
impl crate::Writable for PerdmaclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERDMACLKDIV to value 0"]
impl crate::Resettable for PerdmaclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
