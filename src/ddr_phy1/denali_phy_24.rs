#[doc = "Register `DENALI_PHY_24` reader"]
pub type R = crate::R<DenaliPhy24Spec>;
#[doc = "Register `DENALI_PHY_24` writer"]
pub type W = crate::W<DenaliPhy24Spec>;
#[doc = "Field `PHY_RDLVL_OP_MODE_0` reader - Read leveling algorithm select for slice 0. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode0R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_OP_MODE_0` writer - Read leveling algorithm select for slice 0. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_0` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 0."]
pub type PhyRdlvlRddqsDqObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_0` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 0."]
pub type PhyRdlvlRddqsDqObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_RDLVL_DATA_MASK_0` reader - Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask0R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DATA_MASK_0` writer - Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_0` reader - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
pub type PhyWdqlvlBurstCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_BURST_CNT_0` writer - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
pub type PhyWdqlvlBurstCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 0. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    pub fn phy_rdlvl_op_mode_0(&self) -> PhyRdlvlOpMode0R {
        PhyRdlvlOpMode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_obs_select_0(&self) -> PhyRdlvlRddqsDqObsSelect0R {
        PhyRdlvlRddqsDqObsSelect0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    pub fn phy_rdlvl_data_mask_0(&self) -> PhyRdlvlDataMask0R {
        PhyRdlvlDataMask0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_burst_cnt_0(&self) -> PhyWdqlvlBurstCnt0R {
        PhyWdqlvlBurstCnt0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read leveling algorithm select for slice 0. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_op_mode_0(&mut self) -> PhyRdlvlOpMode0W<DenaliPhy24Spec> {
        PhyRdlvlOpMode0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_obs_select_0(
        &mut self,
    ) -> PhyRdlvlRddqsDqObsSelect0W<DenaliPhy24Spec> {
        PhyRdlvlRddqsDqObsSelect0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_data_mask_0(&mut self) -> PhyRdlvlDataMask0W<DenaliPhy24Spec> {
        PhyRdlvlDataMask0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_burst_cnt_0(&mut self) -> PhyWdqlvlBurstCnt0W<DenaliPhy24Spec> {
        PhyWdqlvlBurstCnt0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy24Spec;
impl crate::RegisterSpec for DenaliPhy24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_24::R`](R) reader structure"]
impl crate::Readable for DenaliPhy24Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_24::W`](W) writer structure"]
impl crate::Writable for DenaliPhy24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_24 to value 0"]
impl crate::Resettable for DenaliPhy24Spec {
    const RESET_VALUE: u32 = 0;
}
