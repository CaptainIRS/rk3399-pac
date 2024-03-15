#[doc = "Register `PKT_SEND_CTL` reader"]
pub type R = crate::R<PktSendCtlSpec>;
#[doc = "Register `PKT_SEND_CTL` writer"]
pub type W = crate::W<PktSendCtlSpec>;
#[doc = "Configurable InfoFrame send enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfEn {
    #[doc = "1: Don't send InfoFrame. Make sure that the IF_TYPE and IF_PKT_DB1~25 Registers had been configured correctly and the IF_UP had been written with 1. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Don't send InfoFrame. Make sure that the IF_TYPE and IF_PKT_DB1~25 Registers had been configured correctly and the IF_UP had been written with 1. This bit's type is R/W."]
    B0 = 0,
}
impl From<IfEn> for bool {
    #[inline(always)]
    fn from(variant: IfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF_EN` reader - Configurable InfoFrame send enable."]
pub type IfEnR = crate::BitReader<IfEn>;
impl IfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IfEn {
        match self.bits {
            true => IfEn::B1,
            false => IfEn::B0,
        }
    }
    #[doc = "Don't send InfoFrame. Make sure that the IF_TYPE and IF_PKT_DB1~25 Registers had been configured correctly and the IF_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IfEn::B1
    }
    #[doc = "Don't send InfoFrame. Make sure that the IF_TYPE and IF_PKT_DB1~25 Registers had been configured correctly and the IF_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IfEn::B0
    }
}
#[doc = "Field `IF_EN` writer - Configurable InfoFrame send enable."]
pub type IfEnW<'a, REG> = crate::BitWriter<'a, REG, IfEn>;
impl<'a, REG> IfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't send InfoFrame. Make sure that the IF_TYPE and IF_PKT_DB1~25 Registers had been configured correctly and the IF_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IfEn::B1)
    }
    #[doc = "Don't send InfoFrame. Make sure that the IF_TYPE and IF_PKT_DB1~25 Registers had been configured correctly and the IF_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IfEn::B0)
    }
}
#[doc = "MPEG InfoFrame send enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MpegInfoEn {
    #[doc = "1: Don't send MPEG InfoFrame. Make sure that the MPEG Packet Content Registers had been configured correctly and the MPEG_INFO_UP had been written with 1. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Don't send MPEG InfoFrame. Make sure that the MPEG Packet Content Registers had been configured correctly and the MPEG_INFO_UP had been written with 1. This bit's type is R/W."]
    B0 = 0,
}
impl From<MpegInfoEn> for bool {
    #[inline(always)]
    fn from(variant: MpegInfoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPEG_INFO_EN` reader - MPEG InfoFrame send enable."]
pub type MpegInfoEnR = crate::BitReader<MpegInfoEn>;
impl MpegInfoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MpegInfoEn {
        match self.bits {
            true => MpegInfoEn::B1,
            false => MpegInfoEn::B0,
        }
    }
    #[doc = "Don't send MPEG InfoFrame. Make sure that the MPEG Packet Content Registers had been configured correctly and the MPEG_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MpegInfoEn::B1
    }
    #[doc = "Don't send MPEG InfoFrame. Make sure that the MPEG Packet Content Registers had been configured correctly and the MPEG_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MpegInfoEn::B0
    }
}
#[doc = "Field `MPEG_INFO_EN` writer - MPEG InfoFrame send enable."]
pub type MpegInfoEnW<'a, REG> = crate::BitWriter<'a, REG, MpegInfoEn>;
impl<'a, REG> MpegInfoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't send MPEG InfoFrame. Make sure that the MPEG Packet Content Registers had been configured correctly and the MPEG_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MpegInfoEn::B1)
    }
    #[doc = "Don't send MPEG InfoFrame. Make sure that the MPEG Packet Content Registers had been configured correctly and the MPEG_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MpegInfoEn::B0)
    }
}
#[doc = "AVI InfoFrame send enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AviInfoEn {
    #[doc = "1: Don't send AVI InfoFrame. Make sure that the AVI Packet Content Registers had been configured correctly and the AVI_INFO_UP had been written with 1. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Don't send AVI InfoFrame. Make sure that the AVI Packet Content Registers had been configured correctly and the AVI_INFO_UP had been written with 1. This bit's type is R/W."]
    B0 = 0,
}
impl From<AviInfoEn> for bool {
    #[inline(always)]
    fn from(variant: AviInfoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVI_INFO_EN` reader - AVI InfoFrame send enable."]
pub type AviInfoEnR = crate::BitReader<AviInfoEn>;
impl AviInfoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AviInfoEn {
        match self.bits {
            true => AviInfoEn::B1,
            false => AviInfoEn::B0,
        }
    }
    #[doc = "Don't send AVI InfoFrame. Make sure that the AVI Packet Content Registers had been configured correctly and the AVI_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AviInfoEn::B1
    }
    #[doc = "Don't send AVI InfoFrame. Make sure that the AVI Packet Content Registers had been configured correctly and the AVI_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AviInfoEn::B0
    }
}
#[doc = "Field `AVI_INFO_EN` writer - AVI InfoFrame send enable."]
pub type AviInfoEnW<'a, REG> = crate::BitWriter<'a, REG, AviInfoEn>;
impl<'a, REG> AviInfoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't send AVI InfoFrame. Make sure that the AVI Packet Content Registers had been configured correctly and the AVI_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AviInfoEn::B1)
    }
    #[doc = "Don't send AVI InfoFrame. Make sure that the AVI Packet Content Registers had been configured correctly and the AVI_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AviInfoEn::B0)
    }
}
#[doc = "Audio InfoFrame send enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioInfoEn {
    #[doc = "1: Don't send Audio InfoFrame. Make sure that the Audio Packet Content Registers had been configured correctly and the AUDIO_INFO_UP had been written with 1. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Don't send Audio InfoFrame. Make sure that the Audio Packet Content Registers had been configured correctly and the AUDIO_INFO_UP had been written with 1. This bit's type is R/W."]
    B0 = 0,
}
impl From<AudioInfoEn> for bool {
    #[inline(always)]
    fn from(variant: AudioInfoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDIO_INFO_EN` reader - Audio InfoFrame send enable."]
pub type AudioInfoEnR = crate::BitReader<AudioInfoEn>;
impl AudioInfoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AudioInfoEn {
        match self.bits {
            true => AudioInfoEn::B1,
            false => AudioInfoEn::B0,
        }
    }
    #[doc = "Don't send Audio InfoFrame. Make sure that the Audio Packet Content Registers had been configured correctly and the AUDIO_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AudioInfoEn::B1
    }
    #[doc = "Don't send Audio InfoFrame. Make sure that the Audio Packet Content Registers had been configured correctly and the AUDIO_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AudioInfoEn::B0
    }
}
#[doc = "Field `AUDIO_INFO_EN` writer - Audio InfoFrame send enable."]
pub type AudioInfoEnW<'a, REG> = crate::BitWriter<'a, REG, AudioInfoEn>;
impl<'a, REG> AudioInfoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't send Audio InfoFrame. Make sure that the Audio Packet Content Registers had been configured correctly and the AUDIO_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AudioInfoEn::B1)
    }
    #[doc = "Don't send Audio InfoFrame. Make sure that the Audio Packet Content Registers had been configured correctly and the AUDIO_INFO_UP had been written with 1. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AudioInfoEn::B0)
    }
}
#[doc = "Configurable InfoFrame content has been updated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfUp {
    #[doc = "1: Don't care. Write 1 to this bit after IF_TYPE and IF_PKT_DB1~25 Registers have been configured as configurable InfoFrame content have been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B1 = 1,
    #[doc = "0: Don't care. Write 1 to this bit after IF_TYPE and IF_PKT_DB1~25 Registers have been configured as configurable InfoFrame content have been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B0 = 0,
}
impl From<IfUp> for bool {
    #[inline(always)]
    fn from(variant: IfUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF_UP` reader - Configurable InfoFrame content has been updated."]
pub type IfUpR = crate::BitReader<IfUp>;
impl IfUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IfUp {
        match self.bits {
            true => IfUp::B1,
            false => IfUp::B0,
        }
    }
    #[doc = "Don't care. Write 1 to this bit after IF_TYPE and IF_PKT_DB1~25 Registers have been configured as configurable InfoFrame content have been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IfUp::B1
    }
    #[doc = "Don't care. Write 1 to this bit after IF_TYPE and IF_PKT_DB1~25 Registers have been configured as configurable InfoFrame content have been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IfUp::B0
    }
}
#[doc = "Field `IF_UP` writer - Configurable InfoFrame content has been updated."]
pub type IfUpW<'a, REG> = crate::BitWriter1C<'a, REG, IfUp>;
impl<'a, REG> IfUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't care. Write 1 to this bit after IF_TYPE and IF_PKT_DB1~25 Registers have been configured as configurable InfoFrame content have been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IfUp::B1)
    }
    #[doc = "Don't care. Write 1 to this bit after IF_TYPE and IF_PKT_DB1~25 Registers have been configured as configurable InfoFrame content have been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IfUp::B0)
    }
}
#[doc = "MPEG InfoFrame content has been updated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MpegInfoUp {
    #[doc = "1: Don't care. Write 1 to this bit after MPEG Packet Content Registers have been configured as MPEG InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B1 = 1,
    #[doc = "0: Don't care. Write 1 to this bit after MPEG Packet Content Registers have been configured as MPEG InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B0 = 0,
}
impl From<MpegInfoUp> for bool {
    #[inline(always)]
    fn from(variant: MpegInfoUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPEG_INFO_UP` reader - MPEG InfoFrame content has been updated."]
pub type MpegInfoUpR = crate::BitReader<MpegInfoUp>;
impl MpegInfoUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MpegInfoUp {
        match self.bits {
            true => MpegInfoUp::B1,
            false => MpegInfoUp::B0,
        }
    }
    #[doc = "Don't care. Write 1 to this bit after MPEG Packet Content Registers have been configured as MPEG InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MpegInfoUp::B1
    }
    #[doc = "Don't care. Write 1 to this bit after MPEG Packet Content Registers have been configured as MPEG InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MpegInfoUp::B0
    }
}
#[doc = "Field `MPEG_INFO_UP` writer - MPEG InfoFrame content has been updated."]
pub type MpegInfoUpW<'a, REG> = crate::BitWriter1C<'a, REG, MpegInfoUp>;
impl<'a, REG> MpegInfoUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't care. Write 1 to this bit after MPEG Packet Content Registers have been configured as MPEG InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MpegInfoUp::B1)
    }
    #[doc = "Don't care. Write 1 to this bit after MPEG Packet Content Registers have been configured as MPEG InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MpegInfoUp::B0)
    }
}
#[doc = "AVI InfoFrame content has been updated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AviInfoUp {
    #[doc = "1: Don't care. Write 1 to this bit after AVI Packet Content Registers have been configured as AVI InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B1 = 1,
    #[doc = "0: Don't care. Write 1 to this bit after AVI Packet Content Registers have been configured as AVI InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B0 = 0,
}
impl From<AviInfoUp> for bool {
    #[inline(always)]
    fn from(variant: AviInfoUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVI_INFO_UP` reader - AVI InfoFrame content has been updated."]
pub type AviInfoUpR = crate::BitReader<AviInfoUp>;
impl AviInfoUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AviInfoUp {
        match self.bits {
            true => AviInfoUp::B1,
            false => AviInfoUp::B0,
        }
    }
    #[doc = "Don't care. Write 1 to this bit after AVI Packet Content Registers have been configured as AVI InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AviInfoUp::B1
    }
    #[doc = "Don't care. Write 1 to this bit after AVI Packet Content Registers have been configured as AVI InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AviInfoUp::B0
    }
}
#[doc = "Field `AVI_INFO_UP` writer - AVI InfoFrame content has been updated."]
pub type AviInfoUpW<'a, REG> = crate::BitWriter1C<'a, REG, AviInfoUp>;
impl<'a, REG> AviInfoUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't care. Write 1 to this bit after AVI Packet Content Registers have been configured as AVI InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AviInfoUp::B1)
    }
    #[doc = "Don't care. Write 1 to this bit after AVI Packet Content Registers have been configured as AVI InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AviInfoUp::B0)
    }
}
#[doc = "Audio InfoFrame content has been updated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioInfoUp {
    #[doc = "1: Don't care. Write 1 to this bit after Audio Packet Content Registers have been configured as Audio InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B1 = 1,
    #[doc = "0: Don't care. Write 1 to this bit after Audio Packet Content Registers have been configured as Audio InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    B0 = 0,
}
impl From<AudioInfoUp> for bool {
    #[inline(always)]
    fn from(variant: AudioInfoUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDIO_INFO_UP` reader - Audio InfoFrame content has been updated."]
pub type AudioInfoUpR = crate::BitReader<AudioInfoUp>;
impl AudioInfoUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AudioInfoUp {
        match self.bits {
            true => AudioInfoUp::B1,
            false => AudioInfoUp::B0,
        }
    }
    #[doc = "Don't care. Write 1 to this bit after Audio Packet Content Registers have been configured as Audio InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AudioInfoUp::B1
    }
    #[doc = "Don't care. Write 1 to this bit after Audio Packet Content Registers have been configured as Audio InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AudioInfoUp::B0
    }
}
#[doc = "Field `AUDIO_INFO_UP` writer - Audio InfoFrame content has been updated."]
pub type AudioInfoUpW<'a, REG> = crate::BitWriter1C<'a, REG, AudioInfoUp>;
impl<'a, REG> AudioInfoUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't care. Write 1 to this bit after Audio Packet Content Registers have been configured as Audio InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AudioInfoUp::B1)
    }
    #[doc = "Don't care. Write 1 to this bit after Audio Packet Content Registers have been configured as Audio InfoFrame content has been updated. This bit's type is R/W. This bit is self cleared after the register configured content has been used to update the InfoFrame."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AudioInfoUp::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Configurable InfoFrame send enable."]
    #[inline(always)]
    pub fn if_en(&self) -> IfEnR {
        IfEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPEG InfoFrame send enable."]
    #[inline(always)]
    pub fn mpeg_info_en(&self) -> MpegInfoEnR {
        MpegInfoEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AVI InfoFrame send enable."]
    #[inline(always)]
    pub fn avi_info_en(&self) -> AviInfoEnR {
        AviInfoEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Audio InfoFrame send enable."]
    #[inline(always)]
    pub fn audio_info_en(&self) -> AudioInfoEnR {
        AudioInfoEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configurable InfoFrame content has been updated."]
    #[inline(always)]
    pub fn if_up(&self) -> IfUpR {
        IfUpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPEG InfoFrame content has been updated."]
    #[inline(always)]
    pub fn mpeg_info_up(&self) -> MpegInfoUpR {
        MpegInfoUpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AVI InfoFrame content has been updated."]
    #[inline(always)]
    pub fn avi_info_up(&self) -> AviInfoUpR {
        AviInfoUpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Audio InfoFrame content has been updated."]
    #[inline(always)]
    pub fn audio_info_up(&self) -> AudioInfoUpR {
        AudioInfoUpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable InfoFrame send enable."]
    #[inline(always)]
    #[must_use]
    pub fn if_en(&mut self) -> IfEnW<PktSendCtlSpec> {
        IfEnW::new(self, 0)
    }
    #[doc = "Bit 1 - MPEG InfoFrame send enable."]
    #[inline(always)]
    #[must_use]
    pub fn mpeg_info_en(&mut self) -> MpegInfoEnW<PktSendCtlSpec> {
        MpegInfoEnW::new(self, 1)
    }
    #[doc = "Bit 2 - AVI InfoFrame send enable."]
    #[inline(always)]
    #[must_use]
    pub fn avi_info_en(&mut self) -> AviInfoEnW<PktSendCtlSpec> {
        AviInfoEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Audio InfoFrame send enable."]
    #[inline(always)]
    #[must_use]
    pub fn audio_info_en(&mut self) -> AudioInfoEnW<PktSendCtlSpec> {
        AudioInfoEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Configurable InfoFrame content has been updated."]
    #[inline(always)]
    #[must_use]
    pub fn if_up(&mut self) -> IfUpW<PktSendCtlSpec> {
        IfUpW::new(self, 4)
    }
    #[doc = "Bit 5 - MPEG InfoFrame content has been updated."]
    #[inline(always)]
    #[must_use]
    pub fn mpeg_info_up(&mut self) -> MpegInfoUpW<PktSendCtlSpec> {
        MpegInfoUpW::new(self, 5)
    }
    #[doc = "Bit 6 - AVI InfoFrame content has been updated."]
    #[inline(always)]
    #[must_use]
    pub fn avi_info_up(&mut self) -> AviInfoUpW<PktSendCtlSpec> {
        AviInfoUpW::new(self, 6)
    }
    #[doc = "Bit 7 - Audio InfoFrame content has been updated."]
    #[inline(always)]
    #[must_use]
    pub fn audio_info_up(&mut self) -> AudioInfoUpW<PktSendCtlSpec> {
        AudioInfoUpW::new(self, 7)
    }
}
#[doc = "Packet Send Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkt_send_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pkt_send_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktSendCtlSpec;
impl crate::RegisterSpec for PktSendCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkt_send_ctl::R`](R) reader structure"]
impl crate::Readable for PktSendCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pkt_send_ctl::W`](W) writer structure"]
impl crate::Writable for PktSendCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf0;
}
#[doc = "`reset()` method sets PKT_SEND_CTL to value 0"]
impl crate::Resettable for PktSendCtlSpec {
    const RESET_VALUE: u32 = 0;
}
