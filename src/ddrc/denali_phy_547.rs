#[doc = "Register `DENALI_PHY_547` reader"]
pub type R = crate::R<DenaliPhy547Spec>;
#[doc = "Register `DENALI_PHY_547` writer"]
pub type W = crate::W<DenaliPhy547Spec>;
#[doc = "Field `PHY_ADR5_CLK_WR_SLAVE_DELAY_0` reader - Address slice slave delay setting for address slice 0."]
pub type PhyAdr5ClkWrSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR5_CLK_WR_SLAVE_DELAY_0` writer - Address slice slave delay setting for address slice 0."]
pub type PhyAdr5ClkWrSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_SW_MASTER_MODE_0` reader - Master delay line override settings for address slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhyAdrSwMasterMode0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SW_MASTER_MODE_0` writer - Master delay line override settings for address slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhyAdrSwMasterMode0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 0."]
    #[inline(always)]
    pub fn phy_adr5_clk_wr_slave_delay_0(&self) -> PhyAdr5ClkWrSlaveDelay0R {
        PhyAdr5ClkWrSlaveDelay0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - Master delay line override settings for address slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    pub fn phy_adr_sw_master_mode_0(&self) -> PhyAdrSwMasterMode0R {
        PhyAdrSwMasterMode0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr5_clk_wr_slave_delay_0(&mut self) -> PhyAdr5ClkWrSlaveDelay0W<DenaliPhy547Spec> {
        PhyAdr5ClkWrSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Master delay line override settings for address slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_master_mode_0(&mut self) -> PhyAdrSwMasterMode0W<DenaliPhy547Spec> {
        PhyAdrSwMasterMode0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_547::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_547::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy547Spec;
impl crate::RegisterSpec for DenaliPhy547Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_547::R`](R) reader structure"]
impl crate::Readable for DenaliPhy547Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_547::W`](W) writer structure"]
impl crate::Writable for DenaliPhy547Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_547 to value 0"]
impl crate::Resettable for DenaliPhy547Spec {
    const RESET_VALUE: u32 = 0;
}
