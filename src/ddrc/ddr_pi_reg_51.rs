#[doc = "Register `DDR_PI_REG_51` reader"]
pub type R = crate::R<DdrPiReg51Spec>;
#[doc = "Field `PI_SW_WRLVL_RESP_3` reader - Indicates write leveling response for data slice 3."]
pub type PiSwWrlvlResp3R = crate::BitReader;
#[doc = "Field `PI_SW_RDLVL_RESP_0` reader - Indicates read leveling response for data slice 0."]
pub type PiSwRdlvlResp0R = crate::FieldReader;
#[doc = "Field `PI_SW_RDLVL_RESP_1` reader - Indicates read leveling response for data slice 1."]
pub type PiSwRdlvlResp1R = crate::FieldReader;
#[doc = "Field `PI_SW_RDLVL_RESP_2` reader - Indicates read leveling response for data slice 2."]
pub type PiSwRdlvlResp2R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates write leveling response for data slice 3."]
    #[inline(always)]
    pub fn pi_sw_wrlvl_resp_3(&self) -> PiSwWrlvlResp3R {
        PiSwWrlvlResp3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates read leveling response for data slice 0."]
    #[inline(always)]
    pub fn pi_sw_rdlvl_resp_0(&self) -> PiSwRdlvlResp0R {
        PiSwRdlvlResp0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Indicates read leveling response for data slice 1."]
    #[inline(always)]
    pub fn pi_sw_rdlvl_resp_1(&self) -> PiSwRdlvlResp1R {
        PiSwRdlvlResp1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Indicates read leveling response for data slice 2."]
    #[inline(always)]
    pub fn pi_sw_rdlvl_resp_2(&self) -> PiSwRdlvlResp2R {
        PiSwRdlvlResp2R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[doc = "DDR PHY Independent Register 51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_51::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg51Spec;
impl crate::RegisterSpec for DdrPiReg51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_51::R`](R) reader structure"]
impl crate::Readable for DdrPiReg51Spec {}
#[doc = "`reset()` method sets DDR_PI_REG_51 to value 0"]
impl crate::Resettable for DdrPiReg51Spec {
    const RESET_VALUE: u32 = 0;
}
