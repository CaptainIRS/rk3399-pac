#[doc = "Register `DDR_DENALI_PHY_462` reader"]
pub type R = crate::R<DdrDenaliPhy462Spec>;
#[doc = "Register `DDR_DENALI_PHY_462` writer"]
pub type W = crate::W<DdrDenaliPhy462Spec>;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_3` reader - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
pub type PhyRddqsLatencyAdjust3R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_3` writer - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
pub type PhyRddqsLatencyAdjust3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_3` reader - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 3."]
pub type PhyWritePathLatAdd3R = crate::FieldReader;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_3` writer - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 3."]
pub type PhyWritePathLatAdd3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_3` reader - Write level delay threshold above which will be considered in previous cycle for slice 3."]
pub type PhyWrlvlDelayEarlyThreshold3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_3` writer - Write level delay threshold above which will be considered in previous cycle for slice 3."]
pub type PhyWrlvlDelayEarlyThreshold3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_latency_adjust_3(&self) -> PhyRddqsLatencyAdjust3R {
        PhyRddqsLatencyAdjust3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 3."]
    #[inline(always)]
    pub fn phy_write_path_lat_add_3(&self) -> PhyWritePathLatAdd3R {
        PhyWritePathLatAdd3R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_early_threshold_3(&self) -> PhyWrlvlDelayEarlyThreshold3R {
        PhyWrlvlDelayEarlyThreshold3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_latency_adjust_3(&mut self) -> PhyRddqsLatencyAdjust3W<DdrDenaliPhy462Spec> {
        PhyRddqsLatencyAdjust3W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_path_lat_add_3(&mut self) -> PhyWritePathLatAdd3W<DdrDenaliPhy462Spec> {
        PhyWritePathLatAdd3W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_early_threshold_3(
        &mut self,
    ) -> PhyWrlvlDelayEarlyThreshold3W<DdrDenaliPhy462Spec> {
        PhyWrlvlDelayEarlyThreshold3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_462::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_462::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy462Spec;
impl crate::RegisterSpec for DdrDenaliPhy462Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_462::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy462Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_462::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy462Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_462 to value 0"]
impl crate::Resettable for DdrDenaliPhy462Spec {
    const RESET_VALUE: u32 = 0;
}
