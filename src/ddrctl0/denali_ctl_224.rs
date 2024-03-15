#[doc = "Register `DENALI_CTL_224` reader"]
pub type R = crate::R<DenaliCtl224Spec>;
#[doc = "Field `SWLVL_RESP_0` reader - Leveling response for data slice 0. READ-ONLY"]
pub type SwlvlResp0R = crate::BitReader;
#[doc = "Field `SWLVL_RESP_1` reader - Leveling response for data slice 1. READ-ONLY"]
pub type SwlvlResp1R = crate::BitReader;
#[doc = "Field `SWLVL_RESP_2` reader - Leveling response for data slice 2. READ-ONLY"]
pub type SwlvlResp2R = crate::BitReader;
#[doc = "Field `SWLVL_RESP_3` reader - Leveling response for data slice 3. READ-ONLY"]
pub type SwlvlResp3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Leveling response for data slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn swlvl_resp_0(&self) -> SwlvlResp0R {
        SwlvlResp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Leveling response for data slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn swlvl_resp_1(&self) -> SwlvlResp1R {
        SwlvlResp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Leveling response for data slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn swlvl_resp_2(&self) -> SwlvlResp2R {
        SwlvlResp2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Leveling response for data slice 3. READ-ONLY"]
    #[inline(always)]
    pub fn swlvl_resp_3(&self) -> SwlvlResp3R {
        SwlvlResp3R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_224::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl224Spec;
impl crate::RegisterSpec for DenaliCtl224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_224::R`](R) reader structure"]
impl crate::Readable for DenaliCtl224Spec {}
#[doc = "`reset()` method sets DENALI_CTL_224 to value 0"]
impl crate::Resettable for DenaliCtl224Spec {
    const RESET_VALUE: u32 = 0;
}
