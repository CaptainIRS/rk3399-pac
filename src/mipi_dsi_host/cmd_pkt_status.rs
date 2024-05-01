#[doc = "Register `CMD_PKT_STATUS` reader"]
pub type R = crate::R<CmdPktStatusSpec>;
#[doc = "Field `GEN_CMD_EMPTY` reader - gen_cmd_empty\n\nThis bit indicates the empty status of the generic command FIFO."]
pub type GenCmdEmptyR = crate::BitReader;
#[doc = "Field `GEN_CMD_FULL` reader - gen_cmd_full\n\nThis bit indicates the full status of the generic command FIFO."]
pub type GenCmdFullR = crate::BitReader;
#[doc = "Field `GEN_PLD_W_EMPTY` reader - gen_pld_w_empty\n\nThis bit indicates the empty status of the generic write payload\n\nFIFO."]
pub type GenPldWEmptyR = crate::BitReader;
#[doc = "Field `GEN_PLD_W_FULL` reader - gen_pld_w_full\n\nThis bit indicates the full status of the generic write payload FIFO."]
pub type GenPldWFullR = crate::BitReader;
#[doc = "Field `GEN_PLD_R_EMPTY` reader - gen_pld_r_empty\n\nThis bit indicates the empty status of the generic read payload\n\nFIFO."]
pub type GenPldREmptyR = crate::BitReader;
#[doc = "Field `GEN_PLD_R_FULL` reader - gen_pld_r_full\n\nThis bit indicates the full status of the generic read payload FIFO."]
pub type GenPldRFullR = crate::BitReader;
#[doc = "Field `GEN_RD_CMD_BUSY` reader - gen_rd_cmd_busy\n\nThis bit is set when a read command is issued and cleared when the\n\nentire response is stored in the FIFO."]
pub type GenRdCmdBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - gen_cmd_empty\n\nThis bit indicates the empty status of the generic command FIFO."]
    #[inline(always)]
    pub fn gen_cmd_empty(&self) -> GenCmdEmptyR {
        GenCmdEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - gen_cmd_full\n\nThis bit indicates the full status of the generic command FIFO."]
    #[inline(always)]
    pub fn gen_cmd_full(&self) -> GenCmdFullR {
        GenCmdFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gen_pld_w_empty\n\nThis bit indicates the empty status of the generic write payload\n\nFIFO."]
    #[inline(always)]
    pub fn gen_pld_w_empty(&self) -> GenPldWEmptyR {
        GenPldWEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gen_pld_w_full\n\nThis bit indicates the full status of the generic write payload FIFO."]
    #[inline(always)]
    pub fn gen_pld_w_full(&self) -> GenPldWFullR {
        GenPldWFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - gen_pld_r_empty\n\nThis bit indicates the empty status of the generic read payload\n\nFIFO."]
    #[inline(always)]
    pub fn gen_pld_r_empty(&self) -> GenPldREmptyR {
        GenPldREmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - gen_pld_r_full\n\nThis bit indicates the full status of the generic read payload FIFO."]
    #[inline(always)]
    pub fn gen_pld_r_full(&self) -> GenPldRFullR {
        GenPldRFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - gen_rd_cmd_busy\n\nThis bit is set when a read command is issued and cleared when the\n\nentire response is stored in the FIFO."]
    #[inline(always)]
    pub fn gen_rd_cmd_busy(&self) -> GenRdCmdBusyR {
        GenRdCmdBusyR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Command Packet Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_pkt_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdPktStatusSpec;
impl crate::RegisterSpec for CmdPktStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_pkt_status::R`](R) reader structure"]
impl crate::Readable for CmdPktStatusSpec {}
#[doc = "`reset()` method sets CMD_PKT_STATUS to value 0"]
impl crate::Resettable for CmdPktStatusSpec {
    const RESET_VALUE: u32 = 0;
}
