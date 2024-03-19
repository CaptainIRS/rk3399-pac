#[doc = "Register `DDR_PI_REG_50` reader"]
pub type R = crate::R<DdrPiReg50Spec>;
#[doc = "Field `PI_SWLVL_OP_DONE` reader - Indicates signals that software leveling is currently in progress.\n\nValue of 1 indicates operation complete."]
pub type PiSwlvlOpDoneR = crate::BitReader;
#[doc = "Field `PI_SW_WRLVL_RESP_0` reader - Indicates write leveling response for data slice 0."]
pub type PiSwWrlvlResp0R = crate::BitReader;
#[doc = "Field `PI_SW_WRLVL_RESP_1` reader - Indicates write leveling response for data slice 1."]
pub type PiSwWrlvlResp1R = crate::BitReader;
#[doc = "Field `PI_SW_WRLVL_RESP_2` reader - Indicates write leveling response for data slice 2."]
pub type PiSwWrlvlResp2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates signals that software leveling is currently in progress.\n\nValue of 1 indicates operation complete."]
    #[inline(always)]
    pub fn pi_swlvl_op_done(&self) -> PiSwlvlOpDoneR {
        PiSwlvlOpDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates write leveling response for data slice 0."]
    #[inline(always)]
    pub fn pi_sw_wrlvl_resp_0(&self) -> PiSwWrlvlResp0R {
        PiSwWrlvlResp0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates write leveling response for data slice 1."]
    #[inline(always)]
    pub fn pi_sw_wrlvl_resp_1(&self) -> PiSwWrlvlResp1R {
        PiSwWrlvlResp1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicates write leveling response for data slice 2."]
    #[inline(always)]
    pub fn pi_sw_wrlvl_resp_2(&self) -> PiSwWrlvlResp2R {
        PiSwWrlvlResp2R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "DDR PHY Independent Register 50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_50::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg50Spec;
impl crate::RegisterSpec for DdrPiReg50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_50::R`](R) reader structure"]
impl crate::Readable for DdrPiReg50Spec {}
#[doc = "`reset()` method sets DDR_PI_REG_50 to value 0"]
impl crate::Resettable for DdrPiReg50Spec {
    const RESET_VALUE: u32 = 0;
}
