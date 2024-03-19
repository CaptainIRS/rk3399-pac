#[doc = "Register `DDR_DENALI_PHY_32` reader"]
pub type R = crate::R<DdrDenaliPhy32Spec>;
#[doc = "Register `DDR_DENALI_PHY_32` writer"]
pub type W = crate::W<DdrDenaliPhy32Spec>;
#[doc = "Field `PHY_USER_PATT4_0` reader - User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
pub type PhyUserPatt4_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_USER_PATT4_0` writer - User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
pub type PhyUserPatt4_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_CALVL_VREF_DRIVING_SLICE_0` reader - Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
pub type PhyCalvlVrefDrivingSlice0R = crate::BitReader;
#[doc = "Field `PHY_CALVL_VREF_DRIVING_SLICE_0` writer - Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
pub type PhyCalvlVrefDrivingSlice0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_MANUAL_CLEAR_0` writer - Manual reset/clear of internal logic for slice 0. Bit (0) initiates manual setup of the read DQS gate. Bit (1) is reset of read entry FIFO pointers. Bit (2) is reset of master delay min/ max lock values. Bit (3) is manual reset of master delay unlock counter. Bit (4) is reset of leveling error bit in the leveling status registers. Bit (5) is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset. WRITE-ONLY"]
pub type ScPhyManualClear0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt4_0(&self) -> PhyUserPatt4_0R {
        PhyUserPatt4_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
    #[inline(always)]
    pub fn phy_calvl_vref_driving_slice_0(&self) -> PhyCalvlVrefDrivingSlice0R {
        PhyCalvlVrefDrivingSlice0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt4_0(&mut self) -> PhyUserPatt4_0W<DdrDenaliPhy32Spec> {
        PhyUserPatt4_0W::new(self, 0)
    }
    #[doc = "Bit 16 - Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_calvl_vref_driving_slice_0(
        &mut self,
    ) -> PhyCalvlVrefDrivingSlice0W<DdrDenaliPhy32Spec> {
        PhyCalvlVrefDrivingSlice0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Manual reset/clear of internal logic for slice 0. Bit (0) initiates manual setup of the read DQS gate. Bit (1) is reset of read entry FIFO pointers. Bit (2) is reset of master delay min/ max lock values. Bit (3) is manual reset of master delay unlock counter. Bit (4) is reset of leveling error bit in the leveling status registers. Bit (5) is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_manual_clear_0(&mut self) -> ScPhyManualClear0W<DdrDenaliPhy32Spec> {
        ScPhyManualClear0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy32Spec;
impl crate::RegisterSpec for DdrDenaliPhy32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_32::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy32Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_32::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_32 to value 0"]
impl crate::Resettable for DdrDenaliPhy32Spec {
    const RESET_VALUE: u32 = 0;
}
