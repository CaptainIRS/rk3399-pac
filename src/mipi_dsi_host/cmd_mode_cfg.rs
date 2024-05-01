#[doc = "Register `CMD_MODE_CFG` reader"]
pub type R = crate::R<CmdModeCfgSpec>;
#[doc = "Register `CMD_MODE_CFG` writer"]
pub type W = crate::W<CmdModeCfgSpec>;
#[doc = "Field `TEAR_FX_EN` reader - tear_fx_en\n\nWhen set to 1, this bit enables the tearing effect acknowledge\n\nrequest."]
pub type TearFxEnR = crate::BitReader;
#[doc = "Field `TEAR_FX_EN` writer - tear_fx_en\n\nWhen set to 1, this bit enables the tearing effect acknowledge\n\nrequest."]
pub type TearFxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_RQST_EN` reader - ack_rqst_en\n\nWhen set to 1, this bit enables the acknowledge request after each\n\npacket transmission."]
pub type AckRqstEnR = crate::BitReader;
#[doc = "Field `ACK_RQST_EN` writer - ack_rqst_en\n\nWhen set to 1, this bit enables the acknowledge request after each\n\npacket transmission."]
pub type AckRqstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_0P_TX` reader - gen_sw_0p_tx\n\nThis bit configures the Generic short write packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSw0pTxR = crate::BitReader;
#[doc = "Field `GEN_SW_0P_TX` writer - gen_sw_0p_tx\n\nThis bit configures the Generic short write packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSw0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_1P_TX` reader - gen_sw_1p_tx\n\nThis bit configures the Generic short write packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSw1pTxR = crate::BitReader;
#[doc = "Field `GEN_SW_1P_TX` writer - gen_sw_1p_tx\n\nThis bit configures the Generic short write packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSw1pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_2P_TX` reader - gen_sw_2p_tx\n\nThis bit configures the Generic short write packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSw2pTxR = crate::BitReader;
#[doc = "Field `GEN_SW_2P_TX` writer - gen_sw_2p_tx\n\nThis bit configures the Generic short write packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSw2pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_0P_TX` reader - gen_sr_0p_tx\n\nThis bit configures the Generic short read packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSr0pTxR = crate::BitReader;
#[doc = "Field `GEN_SR_0P_TX` writer - gen_sr_0p_tx\n\nThis bit configures the Generic short read packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSr0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_1P_TX` reader - gen_sr_1p_tx\n\nThis bit configures the Generic short read packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSr1pTxR = crate::BitReader;
#[doc = "Field `GEN_SR_1P_TX` writer - gen_sr_1p_tx\n\nThis bit configures the Generic short read packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSr1pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_2P_TX` reader - gen_sr_2p_tx\n\nThis bit configures the Generic short read packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSr2pTxR = crate::BitReader;
#[doc = "Field `GEN_SR_2P_TX` writer - gen_sr_2p_tx\n\nThis bit configures the Generic short read packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenSr2pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_LW_TX` reader - gen_lw_tx\n\nThis bit configures the Generic long write packet command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenLwTxR = crate::BitReader;
#[doc = "Field `GEN_LW_TX` writer - gen_lw_tx\n\nThis bit configures the Generic long write packet command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type GenLwTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SW_0P_TX` reader - dcs_sw_0p_tx\n\nThis bit configures the DCS short write packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsSw0pTxR = crate::BitReader;
#[doc = "Field `DCS_SW_0P_TX` writer - dcs_sw_0p_tx\n\nThis bit configures the DCS short write packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsSw0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SW_1P_TX` reader - dcs_sw_1p_tx\n\nThis bit configures the DCS short write packet with one parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsSw1pTxR = crate::BitReader;
#[doc = "Field `DCS_SW_1P_TX` writer - dcs_sw_1p_tx\n\nThis bit configures the DCS short write packet with one parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsSw1pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SR_0P_TX` reader - dcs_sr_0p_tx\n\nThis bit configures the DCS short read packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsSr0pTxR = crate::BitReader;
#[doc = "Field `DCS_SR_0P_TX` writer - dcs_sr_0p_tx\n\nThis bit configures the DCS short read packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsSr0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_LW_TX` reader - dcs_lw_tx\n\nThis bit configures the DCS long write packet command\n\ntransmission\n\ntype:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsLwTxR = crate::BitReader;
#[doc = "Field `DCS_LW_TX` writer - dcs_lw_tx\n\nThis bit configures the DCS long write packet command\n\ntransmission\n\ntype:\n\n■0: High-speed\n\n■1: Low-power"]
pub type DcsLwTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_RD_PKT_SIZE` reader - max_rd_pkt_size\n\nThis bit configures the maximum read packet size command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type MaxRdPktSizeR = crate::BitReader;
#[doc = "Field `MAX_RD_PKT_SIZE` writer - max_rd_pkt_size\n\nThis bit configures the maximum read packet size command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
pub type MaxRdPktSizeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - tear_fx_en\n\nWhen set to 1, this bit enables the tearing effect acknowledge\n\nrequest."]
    #[inline(always)]
    pub fn tear_fx_en(&self) -> TearFxEnR {
        TearFxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ack_rqst_en\n\nWhen set to 1, this bit enables the acknowledge request after each\n\npacket transmission."]
    #[inline(always)]
    pub fn ack_rqst_en(&self) -> AckRqstEnR {
        AckRqstEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - gen_sw_0p_tx\n\nThis bit configures the Generic short write packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn gen_sw_0p_tx(&self) -> GenSw0pTxR {
        GenSw0pTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - gen_sw_1p_tx\n\nThis bit configures the Generic short write packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn gen_sw_1p_tx(&self) -> GenSw1pTxR {
        GenSw1pTxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - gen_sw_2p_tx\n\nThis bit configures the Generic short write packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn gen_sw_2p_tx(&self) -> GenSw2pTxR {
        GenSw2pTxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - gen_sr_0p_tx\n\nThis bit configures the Generic short read packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn gen_sr_0p_tx(&self) -> GenSr0pTxR {
        GenSr0pTxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - gen_sr_1p_tx\n\nThis bit configures the Generic short read packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn gen_sr_1p_tx(&self) -> GenSr1pTxR {
        GenSr1pTxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - gen_sr_2p_tx\n\nThis bit configures the Generic short read packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn gen_sr_2p_tx(&self) -> GenSr2pTxR {
        GenSr2pTxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - gen_lw_tx\n\nThis bit configures the Generic long write packet command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn gen_lw_tx(&self) -> GenLwTxR {
        GenLwTxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - dcs_sw_0p_tx\n\nThis bit configures the DCS short write packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn dcs_sw_0p_tx(&self) -> DcsSw0pTxR {
        DcsSw0pTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - dcs_sw_1p_tx\n\nThis bit configures the DCS short write packet with one parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn dcs_sw_1p_tx(&self) -> DcsSw1pTxR {
        DcsSw1pTxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - dcs_sr_0p_tx\n\nThis bit configures the DCS short read packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn dcs_sr_0p_tx(&self) -> DcsSr0pTxR {
        DcsSr0pTxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - dcs_lw_tx\n\nThis bit configures the DCS long write packet command\n\ntransmission\n\ntype:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn dcs_lw_tx(&self) -> DcsLwTxR {
        DcsLwTxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - max_rd_pkt_size\n\nThis bit configures the maximum read packet size command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    pub fn max_rd_pkt_size(&self) -> MaxRdPktSizeR {
        MaxRdPktSizeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tear_fx_en\n\nWhen set to 1, this bit enables the tearing effect acknowledge\n\nrequest."]
    #[inline(always)]
    #[must_use]
    pub fn tear_fx_en(&mut self) -> TearFxEnW<CmdModeCfgSpec> {
        TearFxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - ack_rqst_en\n\nWhen set to 1, this bit enables the acknowledge request after each\n\npacket transmission."]
    #[inline(always)]
    #[must_use]
    pub fn ack_rqst_en(&mut self) -> AckRqstEnW<CmdModeCfgSpec> {
        AckRqstEnW::new(self, 1)
    }
    #[doc = "Bit 8 - gen_sw_0p_tx\n\nThis bit configures the Generic short write packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sw_0p_tx(&mut self) -> GenSw0pTxW<CmdModeCfgSpec> {
        GenSw0pTxW::new(self, 8)
    }
    #[doc = "Bit 9 - gen_sw_1p_tx\n\nThis bit configures the Generic short write packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sw_1p_tx(&mut self) -> GenSw1pTxW<CmdModeCfgSpec> {
        GenSw1pTxW::new(self, 9)
    }
    #[doc = "Bit 10 - gen_sw_2p_tx\n\nThis bit configures the Generic short write packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sw_2p_tx(&mut self) -> GenSw2pTxW<CmdModeCfgSpec> {
        GenSw2pTxW::new(self, 10)
    }
    #[doc = "Bit 11 - gen_sr_0p_tx\n\nThis bit configures the Generic short read packet with zero\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sr_0p_tx(&mut self) -> GenSr0pTxW<CmdModeCfgSpec> {
        GenSr0pTxW::new(self, 11)
    }
    #[doc = "Bit 12 - gen_sr_1p_tx\n\nThis bit configures the Generic short read packet with one\n\nparameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sr_1p_tx(&mut self) -> GenSr1pTxW<CmdModeCfgSpec> {
        GenSr1pTxW::new(self, 12)
    }
    #[doc = "Bit 13 - gen_sr_2p_tx\n\nThis bit configures the Generic short read packet with two\n\nparameters\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sr_2p_tx(&mut self) -> GenSr2pTxW<CmdModeCfgSpec> {
        GenSr2pTxW::new(self, 13)
    }
    #[doc = "Bit 14 - gen_lw_tx\n\nThis bit configures the Generic long write packet command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn gen_lw_tx(&mut self) -> GenLwTxW<CmdModeCfgSpec> {
        GenLwTxW::new(self, 14)
    }
    #[doc = "Bit 16 - dcs_sw_0p_tx\n\nThis bit configures the DCS short write packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_sw_0p_tx(&mut self) -> DcsSw0pTxW<CmdModeCfgSpec> {
        DcsSw0pTxW::new(self, 16)
    }
    #[doc = "Bit 17 - dcs_sw_1p_tx\n\nThis bit configures the DCS short write packet with one parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_sw_1p_tx(&mut self) -> DcsSw1pTxW<CmdModeCfgSpec> {
        DcsSw1pTxW::new(self, 17)
    }
    #[doc = "Bit 18 - dcs_sr_0p_tx\n\nThis bit configures the DCS short read packet with zero parameter\n\ncommand transmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_sr_0p_tx(&mut self) -> DcsSr0pTxW<CmdModeCfgSpec> {
        DcsSr0pTxW::new(self, 18)
    }
    #[doc = "Bit 19 - dcs_lw_tx\n\nThis bit configures the DCS long write packet command\n\ntransmission\n\ntype:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_lw_tx(&mut self) -> DcsLwTxW<CmdModeCfgSpec> {
        DcsLwTxW::new(self, 19)
    }
    #[doc = "Bit 24 - max_rd_pkt_size\n\nThis bit configures the maximum read packet size command\n\ntransmission type:\n\n■0: High-speed\n\n■1: Low-power"]
    #[inline(always)]
    #[must_use]
    pub fn max_rd_pkt_size(&mut self) -> MaxRdPktSizeW<CmdModeCfgSpec> {
        MaxRdPktSizeW::new(self, 24)
    }
}
#[doc = "Command Mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdModeCfgSpec;
impl crate::RegisterSpec for CmdModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_mode_cfg::R`](R) reader structure"]
impl crate::Readable for CmdModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_mode_cfg::W`](W) writer structure"]
impl crate::Writable for CmdModeCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_MODE_CFG to value 0"]
impl crate::Resettable for CmdModeCfgSpec {
    const RESET_VALUE: u32 = 0;
}
