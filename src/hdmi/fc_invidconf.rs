#[doc = "Register `FC_INVIDCONF` reader"]
pub type R = crate::R<FcInvidconfSpec>;
#[doc = "Register `FC_INVIDCONF` writer"]
pub type W = crate::W<FcInvidconfSpec>;
#[doc = "Field `R_V_BLANK_IN_OSC` reader - Used for CEA861-D modes with fractional Vblank\n\n(for example, modes 5, 6, 7, 10, 11, 20, 21, and\n\n22). For more modes, see the CEA861-D\n\nspecification.\n\nNote: Set this field to 1 for video mode 39,\n\nalthough there is no Vblank oscillation.\n\n1b: Active high"]
pub type RVBlankInOscR = crate::BitReader;
#[doc = "Field `R_V_BLANK_IN_OSC` writer - Used for CEA861-D modes with fractional Vblank\n\n(for example, modes 5, 6, 7, 10, 11, 20, 21, and\n\n22). For more modes, see the CEA861-D\n\nspecification.\n\nNote: Set this field to 1 for video mode 39,\n\nalthough there is no Vblank oscillation.\n\n1b: Active high"]
pub type RVBlankInOscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DVI_MODEZ` reader - Active low\n\n0b: DVI mode selected 1b: HDMI mode selected"]
pub type DviModezR = crate::BitReader;
#[doc = "Field `DVI_MODEZ` writer - Active low\n\n0b: DVI mode selected 1b: HDMI mode selected"]
pub type DviModezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_IN_POLARITY` reader - Data enable input polarity 1b: Active high\n\n0b: Active low"]
pub type DeInPolarityR = crate::BitReader;
#[doc = "Field `DE_IN_POLARITY` writer - Data enable input polarity 1b: Active high\n\n0b: Active low"]
pub type DeInPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC_IN_POLARITY` reader - Hsync input polarity 1b: Active high\n\n0b: Active low"]
pub type HsyncInPolarityR = crate::BitReader;
#[doc = "Field `HSYNC_IN_POLARITY` writer - Hsync input polarity 1b: Active high\n\n0b: Active low"]
pub type HsyncInPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_IN_POLARITY` reader - Vsync input polarity 1b: Active high\n\n0b: Active low"]
pub type VsyncInPolarityR = crate::BitReader;
#[doc = "Field `VSYNC_IN_POLARITY` writer - Vsync input polarity 1b: Active high\n\n0b: Active low"]
pub type VsyncInPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCP_KEEPOUT` reader - Start/stop HDCP keepout window generation 1b:\n\nActive"]
pub type HdcpKeepoutR = crate::BitReader;
#[doc = "Field `HDCP_KEEPOUT` writer - Start/stop HDCP keepout window generation 1b:\n\nActive"]
pub type HdcpKeepoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Used for CEA861-D modes with fractional Vblank\n\n(for example, modes 5, 6, 7, 10, 11, 20, 21, and\n\n22). For more modes, see the CEA861-D\n\nspecification.\n\nNote: Set this field to 1 for video mode 39,\n\nalthough there is no Vblank oscillation.\n\n1b: Active high"]
    #[inline(always)]
    pub fn r_v_blank_in_osc(&self) -> RVBlankInOscR {
        RVBlankInOscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Active low\n\n0b: DVI mode selected 1b: HDMI mode selected"]
    #[inline(always)]
    pub fn dvi_modez(&self) -> DviModezR {
        DviModezR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data enable input polarity 1b: Active high\n\n0b: Active low"]
    #[inline(always)]
    pub fn de_in_polarity(&self) -> DeInPolarityR {
        DeInPolarityR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hsync input polarity 1b: Active high\n\n0b: Active low"]
    #[inline(always)]
    pub fn hsync_in_polarity(&self) -> HsyncInPolarityR {
        HsyncInPolarityR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Vsync input polarity 1b: Active high\n\n0b: Active low"]
    #[inline(always)]
    pub fn vsync_in_polarity(&self) -> VsyncInPolarityR {
        VsyncInPolarityR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start/stop HDCP keepout window generation 1b:\n\nActive"]
    #[inline(always)]
    pub fn hdcp_keepout(&self) -> HdcpKeepoutR {
        HdcpKeepoutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Used for CEA861-D modes with fractional Vblank\n\n(for example, modes 5, 6, 7, 10, 11, 20, 21, and\n\n22). For more modes, see the CEA861-D\n\nspecification.\n\nNote: Set this field to 1 for video mode 39,\n\nalthough there is no Vblank oscillation.\n\n1b: Active high"]
    #[inline(always)]
    #[must_use]
    pub fn r_v_blank_in_osc(&mut self) -> RVBlankInOscW<FcInvidconfSpec> {
        RVBlankInOscW::new(self, 1)
    }
    #[doc = "Bit 3 - Active low\n\n0b: DVI mode selected 1b: HDMI mode selected"]
    #[inline(always)]
    #[must_use]
    pub fn dvi_modez(&mut self) -> DviModezW<FcInvidconfSpec> {
        DviModezW::new(self, 3)
    }
    #[doc = "Bit 4 - Data enable input polarity 1b: Active high\n\n0b: Active low"]
    #[inline(always)]
    #[must_use]
    pub fn de_in_polarity(&mut self) -> DeInPolarityW<FcInvidconfSpec> {
        DeInPolarityW::new(self, 4)
    }
    #[doc = "Bit 5 - Hsync input polarity 1b: Active high\n\n0b: Active low"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_in_polarity(&mut self) -> HsyncInPolarityW<FcInvidconfSpec> {
        HsyncInPolarityW::new(self, 5)
    }
    #[doc = "Bit 6 - Vsync input polarity 1b: Active high\n\n0b: Active low"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_in_polarity(&mut self) -> VsyncInPolarityW<FcInvidconfSpec> {
        VsyncInPolarityW::new(self, 6)
    }
    #[doc = "Bit 7 - Start/stop HDCP keepout window generation 1b:\n\nActive"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_keepout(&mut self) -> HdcpKeepoutW<FcInvidconfSpec> {
        HdcpKeepoutW::new(self, 7)
    }
}
#[doc = "Frame Composer Input Video Configuration and HDCP Keepout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invidconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invidconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInvidconfSpec;
impl crate::RegisterSpec for FcInvidconfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_invidconf::R`](R) reader structure"]
impl crate::Readable for FcInvidconfSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_invidconf::W`](W) writer structure"]
impl crate::Writable for FcInvidconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INVIDCONF to value 0x70"]
impl crate::Resettable for FcInvidconfSpec {
    const RESET_VALUE: u8 = 0x70;
}
