#[doc = "Register `BANK0INFO0` reader"]
pub type R = crate::R<Bank0info0Spec>;
#[doc = "Register `BANK0INFO0` writer"]
pub type W = crate::W<Bank0info0Spec>;
#[doc = "11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Mainsize {
    #[doc = "512: Maximum value of MAINSIZE"]
    Maxsectors = 512,
    #[doc = "8: Minimum value of MAINSIZE"]
    Minsectors = 8,
}
impl From<Mainsize> for u16 {
    #[inline(always)]
    fn from(variant: Mainsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mainsize {
    type Ux = u16;
}
impl crate::IsEnum for Mainsize {}
#[doc = "Field `MAINSIZE` reader - 11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)"]
pub type MainsizeR = crate::FieldReader<Mainsize>;
impl MainsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mainsize> {
        match self.bits {
            512 => Some(Mainsize::Maxsectors),
            8 => Some(Mainsize::Minsectors),
            _ => None,
        }
    }
    #[doc = "Maximum value of MAINSIZE"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Mainsize::Maxsectors
    }
    #[doc = "Minimum value of MAINSIZE"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Mainsize::Minsectors
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)"]
    #[inline(always)]
    pub fn mainsize(&self) -> MainsizeR {
        MainsizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "Bank Info 0 Register for bank 0. Read only register detailing information about Main region size in the bank.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0info0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0info0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bank0info0Spec;
impl crate::RegisterSpec for Bank0info0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank0info0::R`](R) reader structure"]
impl crate::Readable for Bank0info0Spec {}
#[doc = "`write(|w| ..)` method takes [`bank0info0::W`](W) writer structure"]
impl crate::Writable for Bank0info0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANK0INFO0 to value 0x0100"]
impl crate::Resettable for Bank0info0Spec {
    const RESET_VALUE: u32 = 0x0100;
}
