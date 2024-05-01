#[doc = "Register `VID_MODE_CFG` reader"]
pub type R = crate::R<VidModeCfgSpec>;
#[doc = "Register `VID_MODE_CFG` writer"]
pub type W = crate::W<VidModeCfgSpec>;
#[doc = "Field `VID_MODE_TYPE` reader - vid_mode_type\n\nThis field indicates the video mode transmission type as follows:\n\n■00: Non-burst with sync pulses\n\n■01: Non-burst with sync events\n\n■10 and 11: Burst mode"]
pub type VidModeTypeR = crate::BitReader;
#[doc = "Field `VID_MODE_TYPE` writer - vid_mode_type\n\nThis field indicates the video mode transmission type as follows:\n\n■00: Non-burst with sync pulses\n\n■01: Non-burst with sync events\n\n■10 and 11: Burst mode"]
pub type VidModeTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VSA_EN` reader - lp_vsa_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Sync Time (VSA) period when timing allows."]
pub type LpVsaEnR = crate::BitReader;
#[doc = "Field `LP_VSA_EN` writer - lp_vsa_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Sync Time (VSA) period when timing allows."]
pub type LpVsaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VBP_EN` reader - lp_vbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Back Porch (VBP) period when timing allows."]
pub type LpVbpEnR = crate::BitReader;
#[doc = "Field `LP_VBP_EN` writer - lp_vbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Back Porch (VBP) period when timing allows."]
pub type LpVbpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VFP_EN` reader - lp_vfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Front Porch (VFP) period when timing allows."]
pub type LpVfpEnR = crate::BitReader;
#[doc = "Field `LP_VFP_EN` writer - lp_vfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Front Porch (VFP) period when timing allows."]
pub type LpVfpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VACT_EN` reader - lp_vact_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Active (VACT) period when timing allows."]
pub type LpVactEnR = crate::BitReader;
#[doc = "Field `LP_VACT_EN` writer - lp_vact_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Active (VACT) period when timing allows."]
pub type LpVactEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_HBP_EN` reader - lp_hbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Back Porch (HBP) period when timing allows."]
pub type LpHbpEnR = crate::BitReader;
#[doc = "Field `LP_HBP_EN` writer - lp_hbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Back Porch (HBP) period when timing allows."]
pub type LpHbpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_HFP_EN` reader - lp_hfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Front Porch (HFP) period when timing allows."]
pub type LpHfpEnR = crate::BitReader;
#[doc = "Field `LP_HFP_EN` writer - lp_hfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Front Porch (HFP) period when timing allows."]
pub type LpHfpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_BTA_ACK_EN` reader - frame_bta_ack_en\n\nWhen set to 1, this bit enables the request for an acknowledge\n\nresponse at the end of a frame."]
pub type FrameBtaAckEnR = crate::BitReader;
#[doc = "Field `FRAME_BTA_ACK_EN` writer - frame_bta_ack_en\n\nWhen set to 1, this bit enables the request for an acknowledge\n\nresponse at the end of a frame."]
pub type FrameBtaAckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CMD_EN` reader - lp_cmd_en\n\nWhen set to 1, this bit enables the command transmission only in\n\nlowpower\n\nmode."]
pub type LpCmdEnR = crate::BitReader;
#[doc = "Field `LP_CMD_EN` writer - lp_cmd_en\n\nWhen set to 1, this bit enables the command transmission only in\n\nlowpower\n\nmode."]
pub type LpCmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPG_EN` reader - vpg_en\n\nWhen set to 1, this bit enables the video mode pattern generator."]
pub type VpgEnR = crate::BitReader;
#[doc = "Field `VPG_EN` writer - vpg_en\n\nWhen set to 1, this bit enables the video mode pattern generator."]
pub type VpgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "vpg_mode\n\nThis field is to select the pattern:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpgMode {
    #[doc = "0: Color bar (horizontal or vertical)"]
    B0 = 0,
    #[doc = "1: BER pattern (vertical only)"]
    B1 = 1,
}
impl From<VpgMode> for bool {
    #[inline(always)]
    fn from(variant: VpgMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VPG_MODE` reader - vpg_mode\n\nThis field is to select the pattern:"]
pub type VpgModeR = crate::BitReader<VpgMode>;
impl VpgModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpgMode {
        match self.bits {
            false => VpgMode::B0,
            true => VpgMode::B1,
        }
    }
    #[doc = "Color bar (horizontal or vertical)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VpgMode::B0
    }
    #[doc = "BER pattern (vertical only)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VpgMode::B1
    }
}
#[doc = "Field `VPG_MODE` writer - vpg_mode\n\nThis field is to select the pattern:"]
pub type VpgModeW<'a, REG> = crate::BitWriter<'a, REG, VpgMode>;
impl<'a, REG> VpgModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Color bar (horizontal or vertical)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VpgMode::B0)
    }
    #[doc = "BER pattern (vertical only)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VpgMode::B1)
    }
}
#[doc = "vpg_orientation\n\nThis field indicates the color bar orientation as follows:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpgOrientation {
    #[doc = "0: Vertical mode"]
    B0 = 0,
    #[doc = "1: Horizontal mode"]
    B1 = 1,
}
impl From<VpgOrientation> for bool {
    #[inline(always)]
    fn from(variant: VpgOrientation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VPG_ORIENTATION` reader - vpg_orientation\n\nThis field indicates the color bar orientation as follows:"]
pub type VpgOrientationR = crate::BitReader<VpgOrientation>;
impl VpgOrientationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpgOrientation {
        match self.bits {
            false => VpgOrientation::B0,
            true => VpgOrientation::B1,
        }
    }
    #[doc = "Vertical mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VpgOrientation::B0
    }
    #[doc = "Horizontal mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VpgOrientation::B1
    }
}
#[doc = "Field `VPG_ORIENTATION` writer - vpg_orientation\n\nThis field indicates the color bar orientation as follows:"]
pub type VpgOrientationW<'a, REG> = crate::BitWriter<'a, REG, VpgOrientation>;
impl<'a, REG> VpgOrientationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Vertical mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VpgOrientation::B0)
    }
    #[doc = "Horizontal mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VpgOrientation::B1)
    }
}
impl R {
    #[doc = "Bit 0 - vid_mode_type\n\nThis field indicates the video mode transmission type as follows:\n\n■00: Non-burst with sync pulses\n\n■01: Non-burst with sync events\n\n■10 and 11: Burst mode"]
    #[inline(always)]
    pub fn vid_mode_type(&self) -> VidModeTypeR {
        VidModeTypeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - lp_vsa_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Sync Time (VSA) period when timing allows."]
    #[inline(always)]
    pub fn lp_vsa_en(&self) -> LpVsaEnR {
        LpVsaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - lp_vbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Back Porch (VBP) period when timing allows."]
    #[inline(always)]
    pub fn lp_vbp_en(&self) -> LpVbpEnR {
        LpVbpEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - lp_vfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Front Porch (VFP) period when timing allows."]
    #[inline(always)]
    pub fn lp_vfp_en(&self) -> LpVfpEnR {
        LpVfpEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - lp_vact_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Active (VACT) period when timing allows."]
    #[inline(always)]
    pub fn lp_vact_en(&self) -> LpVactEnR {
        LpVactEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - lp_hbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Back Porch (HBP) period when timing allows."]
    #[inline(always)]
    pub fn lp_hbp_en(&self) -> LpHbpEnR {
        LpHbpEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - lp_hfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Front Porch (HFP) period when timing allows."]
    #[inline(always)]
    pub fn lp_hfp_en(&self) -> LpHfpEnR {
        LpHfpEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - frame_bta_ack_en\n\nWhen set to 1, this bit enables the request for an acknowledge\n\nresponse at the end of a frame."]
    #[inline(always)]
    pub fn frame_bta_ack_en(&self) -> FrameBtaAckEnR {
        FrameBtaAckEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - lp_cmd_en\n\nWhen set to 1, this bit enables the command transmission only in\n\nlowpower\n\nmode."]
    #[inline(always)]
    pub fn lp_cmd_en(&self) -> LpCmdEnR {
        LpCmdEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - vpg_en\n\nWhen set to 1, this bit enables the video mode pattern generator."]
    #[inline(always)]
    pub fn vpg_en(&self) -> VpgEnR {
        VpgEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - vpg_mode\n\nThis field is to select the pattern:"]
    #[inline(always)]
    pub fn vpg_mode(&self) -> VpgModeR {
        VpgModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - vpg_orientation\n\nThis field indicates the color bar orientation as follows:"]
    #[inline(always)]
    pub fn vpg_orientation(&self) -> VpgOrientationR {
        VpgOrientationR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - vid_mode_type\n\nThis field indicates the video mode transmission type as follows:\n\n■00: Non-burst with sync pulses\n\n■01: Non-burst with sync events\n\n■10 and 11: Burst mode"]
    #[inline(always)]
    #[must_use]
    pub fn vid_mode_type(&mut self) -> VidModeTypeW<VidModeCfgSpec> {
        VidModeTypeW::new(self, 0)
    }
    #[doc = "Bit 8 - lp_vsa_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Sync Time (VSA) period when timing allows."]
    #[inline(always)]
    #[must_use]
    pub fn lp_vsa_en(&mut self) -> LpVsaEnW<VidModeCfgSpec> {
        LpVsaEnW::new(self, 8)
    }
    #[doc = "Bit 9 - lp_vbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Back Porch (VBP) period when timing allows."]
    #[inline(always)]
    #[must_use]
    pub fn lp_vbp_en(&mut self) -> LpVbpEnW<VidModeCfgSpec> {
        LpVbpEnW::new(self, 9)
    }
    #[doc = "Bit 10 - lp_vfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Front Porch (VFP) period when timing allows."]
    #[inline(always)]
    #[must_use]
    pub fn lp_vfp_en(&mut self) -> LpVfpEnW<VidModeCfgSpec> {
        LpVfpEnW::new(self, 10)
    }
    #[doc = "Bit 11 - lp_vact_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nVertical Active (VACT) period when timing allows."]
    #[inline(always)]
    #[must_use]
    pub fn lp_vact_en(&mut self) -> LpVactEnW<VidModeCfgSpec> {
        LpVactEnW::new(self, 11)
    }
    #[doc = "Bit 12 - lp_hbp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Back Porch (HBP) period when timing allows."]
    #[inline(always)]
    #[must_use]
    pub fn lp_hbp_en(&mut self) -> LpHbpEnW<VidModeCfgSpec> {
        LpHbpEnW::new(self, 12)
    }
    #[doc = "Bit 13 - lp_hfp_en\n\nWhen set to 1, this bit enables the return to low-power inside the\n\nHorizontal Front Porch (HFP) period when timing allows."]
    #[inline(always)]
    #[must_use]
    pub fn lp_hfp_en(&mut self) -> LpHfpEnW<VidModeCfgSpec> {
        LpHfpEnW::new(self, 13)
    }
    #[doc = "Bit 14 - frame_bta_ack_en\n\nWhen set to 1, this bit enables the request for an acknowledge\n\nresponse at the end of a frame."]
    #[inline(always)]
    #[must_use]
    pub fn frame_bta_ack_en(&mut self) -> FrameBtaAckEnW<VidModeCfgSpec> {
        FrameBtaAckEnW::new(self, 14)
    }
    #[doc = "Bit 15 - lp_cmd_en\n\nWhen set to 1, this bit enables the command transmission only in\n\nlowpower\n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd_en(&mut self) -> LpCmdEnW<VidModeCfgSpec> {
        LpCmdEnW::new(self, 15)
    }
    #[doc = "Bit 16 - vpg_en\n\nWhen set to 1, this bit enables the video mode pattern generator."]
    #[inline(always)]
    #[must_use]
    pub fn vpg_en(&mut self) -> VpgEnW<VidModeCfgSpec> {
        VpgEnW::new(self, 16)
    }
    #[doc = "Bit 20 - vpg_mode\n\nThis field is to select the pattern:"]
    #[inline(always)]
    #[must_use]
    pub fn vpg_mode(&mut self) -> VpgModeW<VidModeCfgSpec> {
        VpgModeW::new(self, 20)
    }
    #[doc = "Bit 24 - vpg_orientation\n\nThis field indicates the color bar orientation as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn vpg_orientation(&mut self) -> VpgOrientationW<VidModeCfgSpec> {
        VpgOrientationW::new(self, 24)
    }
}
#[doc = "Video Mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidModeCfgSpec;
impl crate::RegisterSpec for VidModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_mode_cfg::R`](R) reader structure"]
impl crate::Readable for VidModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_mode_cfg::W`](W) writer structure"]
impl crate::Writable for VidModeCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_MODE_CFG to value 0"]
impl crate::Resettable for VidModeCfgSpec {
    const RESET_VALUE: u32 = 0;
}
