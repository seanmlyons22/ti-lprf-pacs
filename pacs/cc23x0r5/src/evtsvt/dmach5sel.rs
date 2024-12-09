#[doc = "Register `DMACH5SEL` reader"]
pub type R = crate::R<Dmach5selSpec>;
#[doc = "Register `DMACH5SEL` writer"]
pub type W = crate::W<Dmach5selSpec>;
#[doc = "2:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipid {
    #[doc = "5: Selects adc0trg as channel source"]
    Adc0trg = 5,
    #[doc = "4: Selects laestrgb as channel source"]
    Laestrgb = 4,
}
impl From<Ipid> for u8 {
    #[inline(always)]
    fn from(variant: Ipid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipid {
    type Ux = u8;
}
impl crate::IsEnum for Ipid {}
#[doc = "Field `IPID` reader - 2:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type IpidR = crate::FieldReader<Ipid>;
impl IpidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ipid> {
        match self.bits {
            5 => Some(Ipid::Adc0trg),
            4 => Some(Ipid::Laestrgb),
            _ => None,
        }
    }
    #[doc = "Selects adc0trg as channel source"]
    #[inline(always)]
    pub fn is_adc0trg(&self) -> bool {
        *self == Ipid::Adc0trg
    }
    #[doc = "Selects laestrgb as channel source"]
    #[inline(always)]
    pub fn is_laestrgb(&self) -> bool {
        *self == Ipid::Laestrgb
    }
}
#[doc = "Field `IPID` writer - 2:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type IpidW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ipid>;
impl<'a, REG> IpidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects adc0trg as channel source"]
    #[inline(always)]
    pub fn adc0trg(self) -> &'a mut crate::W<REG> {
        self.variant(Ipid::Adc0trg)
    }
    #[doc = "Selects laestrgb as channel source"]
    #[inline(always)]
    pub fn laestrgb(self) -> &'a mut crate::W<REG> {
        self.variant(Ipid::Laestrgb)
    }
}
#[doc = "Field `RESERVED3` reader - 28:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ipid(&self) -> IpidR {
        IpidR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:28 - 28:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x03ff_ffff)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ipid(&mut self) -> IpidW<Dmach5selSpec> {
        IpidW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA CH5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach5sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach5sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmach5selSpec;
impl crate::RegisterSpec for Dmach5selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach5sel::R`](R) reader structure"]
impl crate::Readable for Dmach5selSpec {}
#[doc = "`write(|w| ..)` method takes [`dmach5sel::W`](W) writer structure"]
impl crate::Writable for Dmach5selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACH5SEL to value 0"]
impl crate::Resettable for Dmach5selSpec {
    const RESET_VALUE: u32 = 0;
}
