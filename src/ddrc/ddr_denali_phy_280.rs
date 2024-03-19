#[doc = "Register `DDR_DENALI_PHY_280` reader"]
pub type R = crate::R<DdrDenaliPhy280Spec>;
#[doc = "Register `DDR_DENALI_PHY_280` writer"]
pub type W = crate::W<DdrDenaliPhy280Spec>;
#[doc = "Field `PHY_RDLVL_OP_MODE_2` reader - Read leveling algorithm select for slice 2. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode2R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_OP_MODE_2` writer - Read leveling algorithm select for slice 2. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_2` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 2."]
pub type PhyRdlvlRddqsDqObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_2` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 2."]
pub type PhyRdlvlRddqsDqObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_RDLVL_DATA_MASK_2` reader - Per-bit mask for read leveling for slice 2. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask2R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DATA_MASK_2` writer - Per-bit mask for read leveling for slice 2. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_2` reader - Defines the write/read burst length in bytes during the write data leveling sequence for slice 2."]
pub type PhyWdqlvlBurstCnt2R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_2` writer - Defines the write/read burst length in bytes during the write data leveling sequence for slice 2."]
pub type PhyWdqlvlBurstCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 2. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    pub fn phy_rdlvl_op_mode_2(&self) -> PhyRdlvlOpMode2R {
        PhyRdlvlOpMode2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 2."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_obs_select_2(&self) -> PhyRdlvlRddqsDqObsSelect2R {
        PhyRdlvlRddqsDqObsSelect2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 2. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    pub fn phy_rdlvl_data_mask_2(&self) -> PhyRdlvlDataMask2R {
        PhyRdlvlDataMask2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 2."]
    #[inline(always)]
    pub fn phy_wdqlvl_burst_cnt_2(&self) -> PhyWdqlvlBurstCnt2R {
        PhyWdqlvlBurstCnt2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 2. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_op_mode_2(&mut self) -> PhyRdlvlOpMode2W<DdrDenaliPhy280Spec> {
        PhyRdlvlOpMode2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_obs_select_2(
        &mut self,
    ) -> PhyRdlvlRddqsDqObsSelect2W<DdrDenaliPhy280Spec> {
        PhyRdlvlRddqsDqObsSelect2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 2. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_data_mask_2(&mut self) -> PhyRdlvlDataMask2W<DdrDenaliPhy280Spec> {
        PhyRdlvlDataMask2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_burst_cnt_2(&mut self) -> PhyWdqlvlBurstCnt2W<DdrDenaliPhy280Spec> {
        PhyWdqlvlBurstCnt2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_280::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_280::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy280Spec;
impl crate::RegisterSpec for DdrDenaliPhy280Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_280::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy280Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_280::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy280Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_280 to value 0"]
impl crate::Resettable for DdrDenaliPhy280Spec {
    const RESET_VALUE: u32 = 0;
}
