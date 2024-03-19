#[doc = "Register `DDR_DENALI_PHY_408` reader"]
pub type R = crate::R<DdrDenaliPhy408Spec>;
#[doc = "Register `DDR_DENALI_PHY_408` writer"]
pub type W = crate::W<DdrDenaliPhy408Spec>;
#[doc = "Field `PHY_RDLVL_OP_MODE_3` reader - Read leveling algorithm select for slice 3. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode3R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_OP_MODE_3` writer - Read leveling algorithm select for slice 3. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_3` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 3."]
pub type PhyRdlvlRddqsDqObsSelect3R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_3` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 3."]
pub type PhyRdlvlRddqsDqObsSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_RDLVL_DATA_MASK_3` reader - Per-bit mask for read leveling for slice 3. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask3R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DATA_MASK_3` writer - Per-bit mask for read leveling for slice 3. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_3` reader - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
pub type PhyWdqlvlBurstCnt3R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_3` writer - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
pub type PhyWdqlvlBurstCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 3. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    pub fn phy_rdlvl_op_mode_3(&self) -> PhyRdlvlOpMode3R {
        PhyRdlvlOpMode3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_obs_select_3(&self) -> PhyRdlvlRddqsDqObsSelect3R {
        PhyRdlvlRddqsDqObsSelect3R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 3. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    pub fn phy_rdlvl_data_mask_3(&self) -> PhyRdlvlDataMask3R {
        PhyRdlvlDataMask3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
    #[inline(always)]
    pub fn phy_wdqlvl_burst_cnt_3(&self) -> PhyWdqlvlBurstCnt3R {
        PhyWdqlvlBurstCnt3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 3. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_op_mode_3(&mut self) -> PhyRdlvlOpMode3W<DdrDenaliPhy408Spec> {
        PhyRdlvlOpMode3W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_obs_select_3(
        &mut self,
    ) -> PhyRdlvlRddqsDqObsSelect3W<DdrDenaliPhy408Spec> {
        PhyRdlvlRddqsDqObsSelect3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 3. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_data_mask_3(&mut self) -> PhyRdlvlDataMask3W<DdrDenaliPhy408Spec> {
        PhyRdlvlDataMask3W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_burst_cnt_3(&mut self) -> PhyWdqlvlBurstCnt3W<DdrDenaliPhy408Spec> {
        PhyWdqlvlBurstCnt3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_408::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_408::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy408Spec;
impl crate::RegisterSpec for DdrDenaliPhy408Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_408::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy408Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_408::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy408Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_408 to value 0"]
impl crate::Resettable for DdrDenaliPhy408Spec {
    const RESET_VALUE: u32 = 0;
}
