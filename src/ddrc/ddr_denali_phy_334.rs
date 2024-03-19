#[doc = "Register `DDR_DENALI_PHY_334` reader"]
pub type R = crate::R<DdrDenaliPhy334Spec>;
#[doc = "Register `DDR_DENALI_PHY_334` writer"]
pub type W = crate::W<DdrDenaliPhy334Spec>;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_2` reader - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
pub type PhyRddqsLatencyAdjust2R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_2` writer - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
pub type PhyRddqsLatencyAdjust2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_2` reader - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 2."]
pub type PhyWritePathLatAdd2R = crate::FieldReader;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_2` writer - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 2."]
pub type PhyWritePathLatAdd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_2` reader - Write level delay threshold above which will be considered in previous cycle for slice 2."]
pub type PhyWrlvlDelayEarlyThreshold2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_2` writer - Write level delay threshold above which will be considered in previous cycle for slice 2."]
pub type PhyWrlvlDelayEarlyThreshold2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_latency_adjust_2(&self) -> PhyRddqsLatencyAdjust2R {
        PhyRddqsLatencyAdjust2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 2."]
    #[inline(always)]
    pub fn phy_write_path_lat_add_2(&self) -> PhyWritePathLatAdd2R {
        PhyWritePathLatAdd2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_early_threshold_2(&self) -> PhyWrlvlDelayEarlyThreshold2R {
        PhyWrlvlDelayEarlyThreshold2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_latency_adjust_2(&mut self) -> PhyRddqsLatencyAdjust2W<DdrDenaliPhy334Spec> {
        PhyRddqsLatencyAdjust2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_path_lat_add_2(&mut self) -> PhyWritePathLatAdd2W<DdrDenaliPhy334Spec> {
        PhyWritePathLatAdd2W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_early_threshold_2(
        &mut self,
    ) -> PhyWrlvlDelayEarlyThreshold2W<DdrDenaliPhy334Spec> {
        PhyWrlvlDelayEarlyThreshold2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_334::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_334::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy334Spec;
impl crate::RegisterSpec for DdrDenaliPhy334Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_334::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy334Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_334::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy334Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_334 to value 0"]
impl crate::Resettable for DdrDenaliPhy334Spec {
    const RESET_VALUE: u32 = 0;
}
