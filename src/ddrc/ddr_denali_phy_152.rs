#[doc = "Register `DDR_DENALI_PHY_152` reader"]
pub type R = crate::R<DdrDenaliPhy152Spec>;
#[doc = "Register `DDR_DENALI_PHY_152` writer"]
pub type W = crate::W<DdrDenaliPhy152Spec>;
#[doc = "Field `PHY_RDLVL_OP_MODE_1` reader - Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_OP_MODE_1` writer - Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_1` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
pub type PhyRdlvlRddqsDqObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_1` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
pub type PhyRdlvlRddqsDqObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_RDLVL_DATA_MASK_1` reader - Per-bit mask for read leveling for slice 1. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DATA_MASK_1` writer - Per-bit mask for read leveling for slice 1. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_1` reader - Defines the write/read burst length in bytes during the write data leveling sequence for slice 1."]
pub type PhyWdqlvlBurstCnt1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_1` writer - Defines the write/read burst length in bytes during the write data leveling sequence for slice 1."]
pub type PhyWdqlvlBurstCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    pub fn phy_rdlvl_op_mode_1(&self) -> PhyRdlvlOpMode1R {
        PhyRdlvlOpMode1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_obs_select_1(&self) -> PhyRdlvlRddqsDqObsSelect1R {
        PhyRdlvlRddqsDqObsSelect1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 1. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    pub fn phy_rdlvl_data_mask_1(&self) -> PhyRdlvlDataMask1R {
        PhyRdlvlDataMask1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_burst_cnt_1(&self) -> PhyWdqlvlBurstCnt1R {
        PhyWdqlvlBurstCnt1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_op_mode_1(&mut self) -> PhyRdlvlOpMode1W<DdrDenaliPhy152Spec> {
        PhyRdlvlOpMode1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_obs_select_1(
        &mut self,
    ) -> PhyRdlvlRddqsDqObsSelect1W<DdrDenaliPhy152Spec> {
        PhyRdlvlRddqsDqObsSelect1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 1. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_data_mask_1(&mut self) -> PhyRdlvlDataMask1W<DdrDenaliPhy152Spec> {
        PhyRdlvlDataMask1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_burst_cnt_1(&mut self) -> PhyWdqlvlBurstCnt1W<DdrDenaliPhy152Spec> {
        PhyWdqlvlBurstCnt1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_152::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_152::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy152Spec;
impl crate::RegisterSpec for DdrDenaliPhy152Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_152::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy152Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_152::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy152Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_152 to value 0"]
impl crate::Resettable for DdrDenaliPhy152Spec {
    const RESET_VALUE: u32 = 0;
}
