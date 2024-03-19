#[doc = "Register `DDR_DENALI_CTL_224` reader"]
pub type R = crate::R<DdrDenaliCtl224Spec>;
#[doc = "Field `SWLVL_RESP_0` reader - Leveling response for data slice 0."]
pub type SwlvlResp0R = crate::BitReader;
#[doc = "Field `SWLVL_RESP_1` reader - Leveling response for data slice 1."]
pub type SwlvlResp1R = crate::BitReader;
#[doc = "Field `SWLVL_RESP_2` reader - Leveling response for data slice 2."]
pub type SwlvlResp2R = crate::BitReader;
#[doc = "Field `SWLVL_RESP_3` reader - Leveling response for data slice 3."]
pub type SwlvlResp3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Leveling response for data slice 0."]
    #[inline(always)]
    pub fn swlvl_resp_0(&self) -> SwlvlResp0R {
        SwlvlResp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Leveling response for data slice 1."]
    #[inline(always)]
    pub fn swlvl_resp_1(&self) -> SwlvlResp1R {
        SwlvlResp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Leveling response for data slice 2."]
    #[inline(always)]
    pub fn swlvl_resp_2(&self) -> SwlvlResp2R {
        SwlvlResp2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Leveling response for data slice 3."]
    #[inline(always)]
    pub fn swlvl_resp_3(&self) -> SwlvlResp3R {
        SwlvlResp3R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_224::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl224Spec;
impl crate::RegisterSpec for DdrDenaliCtl224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_224::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl224Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_224 to value 0"]
impl crate::Resettable for DdrDenaliCtl224Spec {
    const RESET_VALUE: u32 = 0;
}
