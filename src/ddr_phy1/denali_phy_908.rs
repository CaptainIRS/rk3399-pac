#[doc = "Register `DENALI_PHY_908` reader"]
pub type R = crate::R<DenaliPhy908Spec>;
#[doc = "Register `DENALI_PHY_908` writer"]
pub type W = crate::W<DenaliPhy908Spec>;
#[doc = "Field `PHY_LPDDR3_CS` reader - Alters reset state polarity for LPDDR chip selects."]
pub type PhyLpddr3CsR = crate::BitReader;
#[doc = "Field `PHY_LPDDR3_CS` writer - Alters reset state polarity for LPDDR chip selects."]
pub type PhyLpddr3CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CALVL_RESULT_MASK` reader - Mask bits to ignore ADR slice CA training results from the address slices. Bit (0) correlates to address slice 0 and bit (1) correlates to address slice 1. Set each bit to 1 to ignore."]
pub type PhyCalvlResultMaskR = crate::FieldReader;
#[doc = "Field `PHY_CALVL_RESULT_MASK` writer - Mask bits to ignore ADR slice CA training results from the address slices. Bit (0) correlates to address slice 0 and bit (1) correlates to address slice 1. Set each bit to 1 to ignore."]
pub type PhyCalvlResultMaskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SC_PHY_UPDATE_CLK_CAL_VALUES` writer - Manual update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to trigger."]
pub type ScPhyUpdateClkCalValuesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CONTINUOUS_CLK_CAL_UPDATE` reader - Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
pub type PhyContinuousClkCalUpdateR = crate::BitReader;
#[doc = "Field `PHY_CONTINUOUS_CLK_CAL_UPDATE` writer - Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
pub type PhyContinuousClkCalUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alters reset state polarity for LPDDR chip selects."]
    #[inline(always)]
    pub fn phy_lpddr3_cs(&self) -> PhyLpddr3CsR {
        PhyLpddr3CsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Mask bits to ignore ADR slice CA training results from the address slices. Bit (0) correlates to address slice 0 and bit (1) correlates to address slice 1. Set each bit to 1 to ignore."]
    #[inline(always)]
    pub fn phy_calvl_result_mask(&self) -> PhyCalvlResultMaskR {
        PhyCalvlResultMaskR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 24 - Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
    #[inline(always)]
    pub fn phy_continuous_clk_cal_update(&self) -> PhyContinuousClkCalUpdateR {
        PhyContinuousClkCalUpdateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alters reset state polarity for LPDDR chip selects."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr3_cs(&mut self) -> PhyLpddr3CsW<DenaliPhy908Spec> {
        PhyLpddr3CsW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Mask bits to ignore ADR slice CA training results from the address slices. Bit (0) correlates to address slice 0 and bit (1) correlates to address slice 1. Set each bit to 1 to ignore."]
    #[inline(always)]
    #[must_use]
    pub fn phy_calvl_result_mask(&mut self) -> PhyCalvlResultMaskW<DenaliPhy908Spec> {
        PhyCalvlResultMaskW::new(self, 8)
    }
    #[doc = "Bit 16 - Manual update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_update_clk_cal_values(&mut self) -> ScPhyUpdateClkCalValuesW<DenaliPhy908Spec> {
        ScPhyUpdateClkCalValuesW::new(self, 16)
    }
    #[doc = "Bit 24 - Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
    #[inline(always)]
    #[must_use]
    pub fn phy_continuous_clk_cal_update(
        &mut self,
    ) -> PhyContinuousClkCalUpdateW<DenaliPhy908Spec> {
        PhyContinuousClkCalUpdateW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_908::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_908::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy908Spec;
impl crate::RegisterSpec for DenaliPhy908Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_908::R`](R) reader structure"]
impl crate::Readable for DenaliPhy908Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_908::W`](W) writer structure"]
impl crate::Writable for DenaliPhy908Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_908 to value 0x01"]
impl crate::Resettable for DenaliPhy908Spec {
    const RESET_VALUE: u32 = 0x01;
}
