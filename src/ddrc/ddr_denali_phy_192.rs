#[doc = "Register `DDR_DENALI_PHY_192` reader"]
pub type R = crate::R<DdrDenaliPhy192Spec>;
#[doc = "Register `DDR_DENALI_PHY_192` writer"]
pub type W = crate::W<DdrDenaliPhy192Spec>;
#[doc = "Field `PHY_RDDQ0_SLAVE_DELAY_1` reader - Read DQ0 slave delay setting for slice 1."]
pub type PhyRddq0SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ0_SLAVE_DELAY_1` writer - Read DQ0 slave delay setting for slice 1."]
pub type PhyRddq0SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ1_SLAVE_DELAY_1` reader - Read DQ1 slave delay setting for slice 1."]
pub type PhyRddq1SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ1_SLAVE_DELAY_1` writer - Read DQ1 slave delay setting for slice 1."]
pub type PhyRddq1SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ0 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq0_slave_delay_1(&self) -> PhyRddq0SlaveDelay1R {
        PhyRddq0SlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ1 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq1_slave_delay_1(&self) -> PhyRddq1SlaveDelay1R {
        PhyRddq1SlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ0 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq0_slave_delay_1(&mut self) -> PhyRddq0SlaveDelay1W<DdrDenaliPhy192Spec> {
        PhyRddq0SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ1 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq1_slave_delay_1(&mut self) -> PhyRddq1SlaveDelay1W<DdrDenaliPhy192Spec> {
        PhyRddq1SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_192::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_192::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy192Spec;
impl crate::RegisterSpec for DdrDenaliPhy192Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_192::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy192Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_192::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy192Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_192 to value 0"]
impl crate::Resettable for DdrDenaliPhy192Spec {
    const RESET_VALUE: u32 = 0;
}
