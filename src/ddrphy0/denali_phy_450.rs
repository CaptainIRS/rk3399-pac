#[doc = "Register `DENALI_PHY_450` reader"]
pub type R = crate::R<DenaliPhy450Spec>;
#[doc = "Register `DENALI_PHY_450` writer"]
pub type W = crate::W<DenaliPhy450Spec>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_3` reader - Read DQ4 slave delay setting for slice 3."]
pub type PhyRddq4SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_3` writer - Read DQ4 slave delay setting for slice 3."]
pub type PhyRddq4SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_3` reader - Read DQ5 slave delay setting for slice 3."]
pub type PhyRddq5SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_3` writer - Read DQ5 slave delay setting for slice 3."]
pub type PhyRddq5SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ4 slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddq4_slave_delay_3(&self) -> PhyRddq4SlaveDelay3R {
        PhyRddq4SlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ5 slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddq5_slave_delay_3(&self) -> PhyRddq5SlaveDelay3R {
        PhyRddq5SlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ4 slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq4_slave_delay_3(&mut self) -> PhyRddq4SlaveDelay3W<DenaliPhy450Spec> {
        PhyRddq4SlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ5 slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq5_slave_delay_3(&mut self) -> PhyRddq5SlaveDelay3W<DenaliPhy450Spec> {
        PhyRddq5SlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_450::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_450::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy450Spec;
impl crate::RegisterSpec for DenaliPhy450Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_450::R`](R) reader structure"]
impl crate::Readable for DenaliPhy450Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_450::W`](W) writer structure"]
impl crate::Writable for DenaliPhy450Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_450 to value 0"]
impl crate::Resettable for DenaliPhy450Spec {
    const RESET_VALUE: u32 = 0;
}
