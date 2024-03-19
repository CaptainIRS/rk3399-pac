#[doc = "Register `DDR_DENALI_PHY_449` reader"]
pub type R = crate::R<DdrDenaliPhy449Spec>;
#[doc = "Register `DDR_DENALI_PHY_449` writer"]
pub type W = crate::W<DdrDenaliPhy449Spec>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_3` reader - Read DQ2 slave delay setting for slice 3."]
pub type PhyRddq2SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_3` writer - Read DQ2 slave delay setting for slice 3."]
pub type PhyRddq2SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_3` reader - Read DQ3 slave delay setting for slice 3."]
pub type PhyRddq3SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_3` writer - Read DQ3 slave delay setting for slice 3."]
pub type PhyRddq3SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddq2_slave_delay_3(&self) -> PhyRddq2SlaveDelay3R {
        PhyRddq2SlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddq3_slave_delay_3(&self) -> PhyRddq3SlaveDelay3R {
        PhyRddq3SlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq2_slave_delay_3(&mut self) -> PhyRddq2SlaveDelay3W<DdrDenaliPhy449Spec> {
        PhyRddq2SlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq3_slave_delay_3(&mut self) -> PhyRddq3SlaveDelay3W<DdrDenaliPhy449Spec> {
        PhyRddq3SlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_449::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_449::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy449Spec;
impl crate::RegisterSpec for DdrDenaliPhy449Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_449::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy449Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_449::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy449Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_449 to value 0"]
impl crate::Resettable for DdrDenaliPhy449Spec {
    const RESET_VALUE: u32 = 0;
}
