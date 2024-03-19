#[doc = "Register `DDR_DENALI_PHY_208` reader"]
pub type R = crate::R<DdrDenaliPhy208Spec>;
#[doc = "Register `DDR_DENALI_PHY_208` writer"]
pub type W = crate::W<DdrDenaliPhy208Spec>;
#[doc = "Field `PHY_GTLVL_RDDQS_SLV_DLY_START_1` reader - Initial read DQS gate slave delay setting during gate training for slice 1."]
pub type PhyGtlvlRddqsSlvDlyStart1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_RDDQS_SLV_DLY_START_1` writer - Initial read DQS gate slave delay setting during gate training for slice 1."]
pub type PhyGtlvlRddqsSlvDlyStart1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_LAT_ADJ_START_1` reader - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 1."]
pub type PhyGtlvlLatAdjStart1R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_LAT_ADJ_START_1` writer - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 1."]
pub type PhyGtlvlLatAdjStart1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - Initial read DQS gate slave delay setting during gate training for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_rddqs_slv_dly_start_1(&self) -> PhyGtlvlRddqsSlvDlyStart1R {
        PhyGtlvlRddqsSlvDlyStart1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_lat_adj_start_1(&self) -> PhyGtlvlLatAdjStart1R {
        PhyGtlvlLatAdjStart1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Initial read DQS gate slave delay setting during gate training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_rddqs_slv_dly_start_1(
        &mut self,
    ) -> PhyGtlvlRddqsSlvDlyStart1W<DdrDenaliPhy208Spec> {
        PhyGtlvlRddqsSlvDlyStart1W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_lat_adj_start_1(&mut self) -> PhyGtlvlLatAdjStart1W<DdrDenaliPhy208Spec> {
        PhyGtlvlLatAdjStart1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_208::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_208::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy208Spec;
impl crate::RegisterSpec for DdrDenaliPhy208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_208::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy208Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_208::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy208Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_208 to value 0"]
impl crate::Resettable for DdrDenaliPhy208Spec {
    const RESET_VALUE: u32 = 0;
}
