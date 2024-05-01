#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "YUV 3D denoise status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DnsSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<DnsSts> for bool {
    #[inline(always)]
    fn from(variant: DnsSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNS_STS` reader - YUV 3D denoise status"]
pub type DnsStsR = crate::BitReader<DnsSts>;
impl DnsStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DnsSts {
        match self.bits {
            false => DnsSts::B00,
            true => DnsSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DnsSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DnsSts::B01
    }
}
#[doc = "de-interlace or yuv bypass status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DilSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<DilSts> for bool {
    #[inline(always)]
    fn from(variant: DilSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIL_STS` reader - de-interlace or yuv bypass status"]
pub type DilStsR = crate::BitReader<DilSts>;
impl DilStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DilSts {
        match self.bits {
            false => DilSts::B00,
            true => DilSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DilSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DilSts::B01
    }
}
#[doc = "RGB denoise/enhancement status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DdeSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<DdeSts> for bool {
    #[inline(always)]
    fn from(variant: DdeSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDE_STS` reader - RGB denoise/enhancement status"]
pub type DdeStsR = crate::BitReader<DdeSts>;
impl DdeStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DdeSts {
        match self.bits {
            false => DdeSts::B00,
            true => DdeSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DdeSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DdeSts::B01
    }
}
#[doc = "YUV DMA write status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WyuvSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<WyuvSts> for bool {
    #[inline(always)]
    fn from(variant: WyuvSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WYUV_STS` reader - YUV DMA write status"]
pub type WyuvStsR = crate::BitReader<WyuvSts>;
impl WyuvStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WyuvSts {
        match self.bits {
            false => WyuvSts::B00,
            true => WyuvSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == WyuvSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == WyuvSts::B01
    }
}
#[doc = "YUV DMA read status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RyuvSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<RyuvSts> for bool {
    #[inline(always)]
    fn from(variant: RyuvSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RYUV_STS` reader - YUV DMA read status"]
pub type RyuvStsR = crate::BitReader<RyuvSts>;
impl RyuvStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RyuvSts {
        match self.bits {
            false => RyuvSts::B00,
            true => RyuvSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == RyuvSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == RyuvSts::B01
    }
}
#[doc = "RGB DMA write status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrgbSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<WrgbSts> for bool {
    #[inline(always)]
    fn from(variant: WrgbSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRGB_STS` reader - RGB DMA write status"]
pub type WrgbStsR = crate::BitReader<WrgbSts>;
impl WrgbStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrgbSts {
        match self.bits {
            false => WrgbSts::B00,
            true => WrgbSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == WrgbSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == WrgbSts::B01
    }
}
#[doc = "RGB DMA read status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RrgbSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<RrgbSts> for bool {
    #[inline(always)]
    fn from(variant: RrgbSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRGB_STS` reader - RGB DMA read status"]
pub type RrgbStsR = crate::BitReader<RrgbSts>;
impl RrgbStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RrgbSts {
        match self.bits {
            false => RrgbSts::B00,
            true => RrgbSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == RrgbSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == RrgbSts::B01
    }
}
#[doc = "vop direct path status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoiSts {
    #[doc = "0: idle"]
    B00 = 0,
    #[doc = "1: working"]
    B01 = 1,
}
impl From<VoiSts> for bool {
    #[inline(always)]
    fn from(variant: VoiSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOI_STS` reader - vop direct path status"]
pub type VoiStsR = crate::BitReader<VoiSts>;
impl VoiStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VoiSts {
        match self.bits {
            false => VoiSts::B00,
            true => VoiSts::B01,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == VoiSts::B00
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == VoiSts::B01
    }
}
#[doc = "Field `WYUV_IDLE_ACK` reader - YUV write DMA idle acknowlege"]
pub type WyuvIdleAckR = crate::BitReader;
#[doc = "Field `WYUV_IDLE_ACK` writer - YUV write DMA idle acknowlege"]
pub type WyuvIdleAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RYUV_IDLE_ACK` reader - YUV read DMA idle acknowlege"]
pub type RyuvIdleAckR = crate::BitReader;
#[doc = "Field `RYUV_IDLE_ACK` writer - YUV read DMA idle acknowlege"]
pub type RyuvIdleAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRGB_IDLE_ACK` reader - RGB write DMA idle acknowlege"]
pub type WrgbIdleAckR = crate::BitReader;
#[doc = "Field `WRGB_IDLE_ACK` writer - RGB write DMA idle acknowlege"]
pub type WrgbIdleAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRGB_IDLE_ACK` reader - RGB read DMA idle acknowlege"]
pub type RrgbIdleAckR = crate::BitReader;
#[doc = "Field `RRGB_IDLE_ACK` writer - RGB read DMA idle acknowlege"]
pub type RrgbIdleAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - YUV 3D denoise status"]
    #[inline(always)]
    pub fn dns_sts(&self) -> DnsStsR {
        DnsStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - de-interlace or yuv bypass status"]
    #[inline(always)]
    pub fn dil_sts(&self) -> DilStsR {
        DilStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RGB denoise/enhancement status"]
    #[inline(always)]
    pub fn dde_sts(&self) -> DdeStsR {
        DdeStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - YUV DMA write status"]
    #[inline(always)]
    pub fn wyuv_sts(&self) -> WyuvStsR {
        WyuvStsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - YUV DMA read status"]
    #[inline(always)]
    pub fn ryuv_sts(&self) -> RyuvStsR {
        RyuvStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RGB DMA write status"]
    #[inline(always)]
    pub fn wrgb_sts(&self) -> WrgbStsR {
        WrgbStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RGB DMA read status"]
    #[inline(always)]
    pub fn rrgb_sts(&self) -> RrgbStsR {
        RrgbStsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - vop direct path status"]
    #[inline(always)]
    pub fn voi_sts(&self) -> VoiStsR {
        VoiStsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - YUV write DMA idle acknowlege"]
    #[inline(always)]
    pub fn wyuv_idle_ack(&self) -> WyuvIdleAckR {
        WyuvIdleAckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - YUV read DMA idle acknowlege"]
    #[inline(always)]
    pub fn ryuv_idle_ack(&self) -> RyuvIdleAckR {
        RyuvIdleAckR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RGB write DMA idle acknowlege"]
    #[inline(always)]
    pub fn wrgb_idle_ack(&self) -> WrgbIdleAckR {
        WrgbIdleAckR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RGB read DMA idle acknowlege"]
    #[inline(always)]
    pub fn rrgb_idle_ack(&self) -> RrgbIdleAckR {
        RrgbIdleAckR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - YUV write DMA idle acknowlege"]
    #[inline(always)]
    #[must_use]
    pub fn wyuv_idle_ack(&mut self) -> WyuvIdleAckW<StatusSpec> {
        WyuvIdleAckW::new(self, 16)
    }
    #[doc = "Bit 17 - YUV read DMA idle acknowlege"]
    #[inline(always)]
    #[must_use]
    pub fn ryuv_idle_ack(&mut self) -> RyuvIdleAckW<StatusSpec> {
        RyuvIdleAckW::new(self, 17)
    }
    #[doc = "Bit 18 - RGB write DMA idle acknowlege"]
    #[inline(always)]
    #[must_use]
    pub fn wrgb_idle_ack(&mut self) -> WrgbIdleAckW<StatusSpec> {
        WrgbIdleAckW::new(self, 18)
    }
    #[doc = "Bit 19 - RGB read DMA idle acknowlege"]
    #[inline(always)]
    #[must_use]
    pub fn rrgb_idle_ack(&mut self) -> RrgbIdleAckW<StatusSpec> {
        RrgbIdleAckW::new(self, 19)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
