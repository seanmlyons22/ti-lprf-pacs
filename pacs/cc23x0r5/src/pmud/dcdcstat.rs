#[doc = "Register `DCDCSTAT` reader"]
pub type R = crate::R<DcdcstatSpec>;
#[doc = "Register `DCDCSTAT` writer"]
pub type W = crate::W<DcdcstatSpec>;
#[doc = "6:0\\]
This indicates DCDC load meter output value in percentage scale. Valid range is 0x1 to 0x64.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Load {
    #[doc = "127: Maximum value"]
    Max = 127,
    #[doc = "0: Minimum value"]
    Min = 0,
}
impl From<Load> for u8 {
    #[inline(always)]
    fn from(variant: Load) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Load {
    type Ux = u8;
}
impl crate::IsEnum for Load {}
#[doc = "Field `LOAD` reader - 6:0\\]
This indicates DCDC load meter output value in percentage scale. Valid range is 0x1 to 0x64."]
pub type LoadR = crate::FieldReader<Load>;
impl LoadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Load> {
        match self.bits {
            127 => Some(Load::Max),
            0 => Some(Load::Min),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Load::Max
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Load::Min
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "10:8\\]
DCDC IPEAK value. This value is same as what is programmed in SYS0.TDCDC.IPEAK when adaptive IPEAK adjustment scheme is not enabled and it shows current IPEAK value applied by hardware when adaptive IPEAK adjustment scheme is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipeak {
    #[doc = "7: Maximum value"]
    Max = 7,
    #[doc = "0: Minimum value"]
    Min = 0,
}
impl From<Ipeak> for u8 {
    #[inline(always)]
    fn from(variant: Ipeak) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipeak {
    type Ux = u8;
}
impl crate::IsEnum for Ipeak {}
#[doc = "Field `IPEAK` reader - 10:8\\]
DCDC IPEAK value. This value is same as what is programmed in SYS0.TDCDC.IPEAK when adaptive IPEAK adjustment scheme is not enabled and it shows current IPEAK value applied by hardware when adaptive IPEAK adjustment scheme is enabled."]
pub type IpeakR = crate::FieldReader<Ipeak>;
impl IpeakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ipeak> {
        match self.bits {
            7 => Some(Ipeak::Max),
            0 => Some(Ipeak::Min),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Ipeak::Max
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Ipeak::Min
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
This indicates DCDC load meter output value in percentage scale. Valid range is 0x1 to 0x64."]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
DCDC IPEAK value. This value is same as what is programmed in SYS0.TDCDC.IPEAK when adaptive IPEAK adjustment scheme is not enabled and it shows current IPEAK value applied by hardware when adaptive IPEAK adjustment scheme is enabled."]
    #[inline(always)]
    pub fn ipeak(&self) -> IpeakR {
        IpeakR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {}
#[doc = "DCDC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcstatSpec;
impl crate::RegisterSpec for DcdcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcstat::R`](R) reader structure"]
impl crate::Readable for DcdcstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdcstat::W`](W) writer structure"]
impl crate::Writable for DcdcstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCSTAT to value 0"]
impl crate::Resettable for DcdcstatSpec {
    const RESET_VALUE: u32 = 0;
}
