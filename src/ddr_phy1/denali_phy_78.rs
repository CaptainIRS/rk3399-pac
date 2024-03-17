#[doc = "Register `DENALI_PHY_78` reader"]
pub type R = crate::R<DenaliPhy78Spec>;
#[doc = "Register `DENALI_PHY_78` writer"]
pub type W = crate::W<DenaliPhy78Spec>;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_0` reader - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyRddqsLatencyAdjust0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_0` writer - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyRddqsLatencyAdjust0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_0` reader - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
pub type PhyWritePathLatAdd0R = crate::FieldReader;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_0` writer - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
pub type PhyWritePathLatAdd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_0` reader - Write level delay threshold above which will be considered in previous cycle for slice 0."]
pub type PhyWrlvlDelayEarlyThreshold0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_0` writer - Write level delay threshold above which will be considered in previous cycle for slice 0."]
pub type PhyWrlvlDelayEarlyThreshold0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_latency_adjust_0(&self) -> PhyRddqsLatencyAdjust0R {
        PhyRddqsLatencyAdjust0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
    #[inline(always)]
    pub fn phy_write_path_lat_add_0(&self) -> PhyWritePathLatAdd0R {
        PhyWritePathLatAdd0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_early_threshold_0(&self) -> PhyWrlvlDelayEarlyThreshold0R {
        PhyWrlvlDelayEarlyThreshold0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_latency_adjust_0(&mut self) -> PhyRddqsLatencyAdjust0W<DenaliPhy78Spec> {
        PhyRddqsLatencyAdjust0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_path_lat_add_0(&mut self) -> PhyWritePathLatAdd0W<DenaliPhy78Spec> {
        PhyWritePathLatAdd0W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Write level delay threshold above which will be considered in previous cycle for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_early_threshold_0(
        &mut self,
    ) -> PhyWrlvlDelayEarlyThreshold0W<DenaliPhy78Spec> {
        PhyWrlvlDelayEarlyThreshold0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_78::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_78::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy78Spec;
impl crate::RegisterSpec for DenaliPhy78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_78::R`](R) reader structure"]
impl crate::Readable for DenaliPhy78Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_78::W`](W) writer structure"]
impl crate::Writable for DenaliPhy78Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_78 to value 0"]
impl crate::Resettable for DenaliPhy78Spec {
    const RESET_VALUE: u32 = 0;
}
