#[doc = "Register `SOC_CON26` reader"]
pub type R = crate::R<SocCon26Spec>;
#[doc = "Register `SOC_CON26` writer"]
pub type W = crate::W<SocCon26Spec>;
#[doc = "Field `EDP_TX_BSCAN_EN` reader - edp_tx_bscan_en bit control"]
pub type EdpTxBscanEnR = crate::BitReader;
#[doc = "Field `EDP_TX_BSCAN_EN` writer - edp_tx_bscan_en bit control"]
pub type EdpTxBscanEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCP22_SRC_SEL` reader - hdcp22_src_sel bit control"]
pub type Hdcp22SrcSelR = crate::BitReader;
#[doc = "Field `HDCP22_SRC_SEL` writer - hdcp22_src_sel bit control"]
pub type Hdcp22SrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY_DP_SEL` reader - uphy_dp_sel bit control"]
pub type UphyDpSelR = crate::BitReader;
#[doc = "Field `UPHY_DP_SEL` writer - uphy_dp_sel bit control"]
pub type UphyDpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPTX_LANE_SEL` reader - dptx_lane_sel bit control"]
pub type DptxLaneSelR = crate::FieldReader;
#[doc = "Field `DPTX_LANE_SEL` writer - dptx_lane_sel bit control"]
pub type DptxLaneSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FORCE_DP_XT_OCDHALTONRESET` reader - force_dp_xt_ocdhaltonreset bit control"]
pub type ForceDpXtOcdhaltonresetR = crate::BitReader;
#[doc = "Field `FORCE_DP_XT_OCDHALTONRESET` writer - force_dp_xt_ocdhaltonreset bit control"]
pub type ForceDpXtOcdhaltonresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPTX_HPD_SEL` reader - dptx_hpd_sel bit control"]
pub type DptxHpdSelR = crate::FieldReader;
#[doc = "Field `DPTX_HPD_SEL` writer - dptx_hpd_sel bit control"]
pub type DptxHpdSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - edp_tx_bscan_en bit control"]
    #[inline(always)]
    pub fn edp_tx_bscan_en(&self) -> EdpTxBscanEnR {
        EdpTxBscanEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - hdcp22_src_sel bit control"]
    #[inline(always)]
    pub fn hdcp22_src_sel(&self) -> Hdcp22SrcSelR {
        Hdcp22SrcSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - uphy_dp_sel bit control"]
    #[inline(always)]
    pub fn uphy_dp_sel(&self) -> UphyDpSelR {
        UphyDpSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - dptx_lane_sel bit control"]
    #[inline(always)]
    pub fn dptx_lane_sel(&self) -> DptxLaneSelR {
        DptxLaneSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - force_dp_xt_ocdhaltonreset bit control"]
    #[inline(always)]
    pub fn force_dp_xt_ocdhaltonreset(&self) -> ForceDpXtOcdhaltonresetR {
        ForceDpXtOcdhaltonresetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - dptx_hpd_sel bit control"]
    #[inline(always)]
    pub fn dptx_hpd_sel(&self) -> DptxHpdSelR {
        DptxHpdSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - edp_tx_bscan_en bit control"]
    #[inline(always)]
    #[must_use]
    pub fn edp_tx_bscan_en(&mut self) -> EdpTxBscanEnW<SocCon26Spec> {
        EdpTxBscanEnW::new(self, 0)
    }
    #[doc = "Bit 2 - hdcp22_src_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_src_sel(&mut self) -> Hdcp22SrcSelW<SocCon26Spec> {
        Hdcp22SrcSelW::new(self, 2)
    }
    #[doc = "Bit 3 - uphy_dp_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn uphy_dp_sel(&mut self) -> UphyDpSelW<SocCon26Spec> {
        UphyDpSelW::new(self, 3)
    }
    #[doc = "Bits 4:7 - dptx_lane_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dptx_lane_sel(&mut self) -> DptxLaneSelW<SocCon26Spec> {
        DptxLaneSelW::new(self, 4)
    }
    #[doc = "Bit 8 - force_dp_xt_ocdhaltonreset bit control"]
    #[inline(always)]
    #[must_use]
    pub fn force_dp_xt_ocdhaltonreset(&mut self) -> ForceDpXtOcdhaltonresetW<SocCon26Spec> {
        ForceDpXtOcdhaltonresetW::new(self, 8)
    }
    #[doc = "Bits 12:13 - dptx_hpd_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dptx_hpd_sel(&mut self) -> DptxHpdSelW<SocCon26Spec> {
        DptxHpdSelW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon26Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon26Spec;
impl crate::RegisterSpec for SocCon26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con26::R`](R) reader structure"]
impl crate::Readable for SocCon26Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_con26::W`](W) writer structure"]
impl crate::Writable for SocCon26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON26 to value 0x0110"]
impl crate::Resettable for SocCon26Spec {
    const RESET_VALUE: u32 = 0x0110;
}
