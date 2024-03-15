#[doc = "Register `DENALI_PHY_206` reader"]
pub type R = crate::R<DenaliPhy206Spec>;
#[doc = "Register `DENALI_PHY_206` writer"]
pub type W = crate::W<DenaliPhy206Spec>;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_1` reader - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
pub type PhyRddqsLatencyAdjust1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_1` writer - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
pub type PhyRddqsLatencyAdjust1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_1` reader - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
pub type PhyWritePathLatAdd1R = crate::FieldReader;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_1` writer - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
pub type PhyWritePathLatAdd1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_1` reader - Write level delay threshold above which will be considered in previous cycle for slice 1."]
pub type PhyWrlvlDelayEarlyThreshold1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_1` writer - Write level delay threshold above which will be considered in previous cycle for slice 1."]
pub type PhyWrlvlDelayEarlyThreshold1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_latency_adjust_1(&self) -> PhyRddqsLatencyAdjust1R {
        PhyRddqsLatencyAdjust1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
    #[inline(always)]
    pub fn phy_write_path_lat_add_1(&self) -> PhyWritePathLatAdd1R {
        PhyWritePathLatAdd1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_early_threshold_1(&self) -> PhyWrlvlDelayEarlyThreshold1R {
        PhyWrlvlDelayEarlyThreshold1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_latency_adjust_1(&mut self) -> PhyRddqsLatencyAdjust1W<DenaliPhy206Spec> {
        PhyRddqsLatencyAdjust1W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_path_lat_add_1(&mut self) -> PhyWritePathLatAdd1W<DenaliPhy206Spec> {
        PhyWritePathLatAdd1W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_early_threshold_1(
        &mut self,
    ) -> PhyWrlvlDelayEarlyThreshold1W<DenaliPhy206Spec> {
        PhyWrlvlDelayEarlyThreshold1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_206::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_206::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy206Spec;
impl crate::RegisterSpec for DenaliPhy206Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_206::R`](R) reader structure"]
impl crate::Readable for DenaliPhy206Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_206::W`](W) writer structure"]
impl crate::Writable for DenaliPhy206Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_206 to value 0"]
impl crate::Resettable for DenaliPhy206Spec {
    const RESET_VALUE: u32 = 0;
}
