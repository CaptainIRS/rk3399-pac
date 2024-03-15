#[doc = "Register `DENALI_PHY_64` reader"]
pub type R = crate::R<DenaliPhy64Spec>;
#[doc = "Register `DENALI_PHY_64` writer"]
pub type W = crate::W<DenaliPhy64Spec>;
#[doc = "Field `PHY_RDDQ0_SLAVE_DELAY_0` reader - Read DQ0 slave delay setting for slice 0."]
pub type PhyRddq0SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ0_SLAVE_DELAY_0` writer - Read DQ0 slave delay setting for slice 0."]
pub type PhyRddq0SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ1_SLAVE_DELAY_0` reader - Read DQ1 slave delay setting for slice 0."]
pub type PhyRddq1SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ1_SLAVE_DELAY_0` writer - Read DQ1 slave delay setting for slice 0."]
pub type PhyRddq1SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ0 slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddq0_slave_delay_0(&self) -> PhyRddq0SlaveDelay0R {
        PhyRddq0SlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ1 slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddq1_slave_delay_0(&self) -> PhyRddq1SlaveDelay0R {
        PhyRddq1SlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ0 slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq0_slave_delay_0(&mut self) -> PhyRddq0SlaveDelay0W<DenaliPhy64Spec> {
        PhyRddq0SlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ1 slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq1_slave_delay_0(&mut self) -> PhyRddq1SlaveDelay0W<DenaliPhy64Spec> {
        PhyRddq1SlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy64Spec;
impl crate::RegisterSpec for DenaliPhy64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_64::R`](R) reader structure"]
impl crate::Readable for DenaliPhy64Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_64::W`](W) writer structure"]
impl crate::Writable for DenaliPhy64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_64 to value 0"]
impl crate::Resettable for DenaliPhy64Spec {
    const RESET_VALUE: u32 = 0;
}
