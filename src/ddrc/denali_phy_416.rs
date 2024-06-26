#[doc = "Register `DENALI_PHY_416` reader"]
pub type R = crate::R<DenaliPhy416Spec>;
#[doc = "Register `DENALI_PHY_416` writer"]
pub type W = crate::W<DenaliPhy416Spec>;
#[doc = "Field `PHY_USER_PATT4_3` reader - User-defined pattern to be used during write data leveling for slice 3. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
pub type PhyUserPatt4_3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_USER_PATT4_3` writer - User-defined pattern to be used during write data leveling for slice 3. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
pub type PhyUserPatt4_3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_CALVL_VREF_DRIVING_SLICE_3` reader - Indicates if slice 3 is used to drive the VREF value to the device during CA training."]
pub type PhyCalvlVrefDrivingSlice3R = crate::BitReader;
#[doc = "Field `PHY_CALVL_VREF_DRIVING_SLICE_3` writer - Indicates if slice 3 is used to drive the VREF value to the device during CA training."]
pub type PhyCalvlVrefDrivingSlice3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_MANUAL_CLEAR_3` writer - Manual reset/clear of internal logic for slice 3. Bit (0) initiates manual setup of the read DQS gate. Bit (1) is reset of read entry FIFO pointers. Bit (2) is reset of master delay min/ max lock values. Bit (3) is manual reset of master delay unlock counter. Bit (4) is reset of leveling error bit in the leveling status registers. Bit (5) is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset."]
pub type ScPhyManualClear3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - User-defined pattern to be used during write data leveling for slice 3. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt4_3(&self) -> PhyUserPatt4_3R {
        PhyUserPatt4_3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Indicates if slice 3 is used to drive the VREF value to the device during CA training."]
    #[inline(always)]
    pub fn phy_calvl_vref_driving_slice_3(&self) -> PhyCalvlVrefDrivingSlice3R {
        PhyCalvlVrefDrivingSlice3R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User-defined pattern to be used during write data leveling for slice 3. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt4_3(&mut self) -> PhyUserPatt4_3W<DenaliPhy416Spec> {
        PhyUserPatt4_3W::new(self, 0)
    }
    #[doc = "Bit 16 - Indicates if slice 3 is used to drive the VREF value to the device during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_calvl_vref_driving_slice_3(
        &mut self,
    ) -> PhyCalvlVrefDrivingSlice3W<DenaliPhy416Spec> {
        PhyCalvlVrefDrivingSlice3W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Manual reset/clear of internal logic for slice 3. Bit (0) initiates manual setup of the read DQS gate. Bit (1) is reset of read entry FIFO pointers. Bit (2) is reset of master delay min/ max lock values. Bit (3) is manual reset of master delay unlock counter. Bit (4) is reset of leveling error bit in the leveling status registers. Bit (5) is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_manual_clear_3(&mut self) -> ScPhyManualClear3W<DenaliPhy416Spec> {
        ScPhyManualClear3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_416::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_416::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy416Spec;
impl crate::RegisterSpec for DenaliPhy416Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_416::R`](R) reader structure"]
impl crate::Readable for DenaliPhy416Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_416::W`](W) writer structure"]
impl crate::Writable for DenaliPhy416Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_416 to value 0"]
impl crate::Resettable for DenaliPhy416Spec {
    const RESET_VALUE: u32 = 0;
}
