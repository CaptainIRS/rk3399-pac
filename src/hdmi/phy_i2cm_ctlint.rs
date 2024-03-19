#[doc = "Register `PHY_I2CM_CTLINT` reader"]
pub type R = crate::R<PhyI2cmCtlintSpec>;
#[doc = "Register `PHY_I2CM_CTLINT` writer"]
pub type W = crate::W<PhyI2cmCtlintSpec>;
#[doc = "Field `ARBITRATION_INTERRUPT` reader - Arbitration error interrupt bit\n\n{arbitration_interrupt = (arbitration_mask==0b) &amp;&amp;\n\n(arbitration_status==arbitration_pol)}\n\nNote: This bit field is read by the sticky bits present\n\non the ih_i2cmphy_stat0 register."]
pub type ArbitrationInterruptR = crate::BitReader;
#[doc = "Field `ARBITRATION_MASK` reader - Arbitration error interrupt mask signal."]
pub type ArbitrationMaskR = crate::BitReader;
#[doc = "Field `ARBITRATION_MASK` writer - Arbitration error interrupt mask signal."]
pub type ArbitrationMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_POL` reader - Arbitration error interrupt polarity configuration."]
pub type ArbitrationPolR = crate::BitReader;
#[doc = "Field `ARBITRATION_POL` writer - Arbitration error interrupt polarity configuration."]
pub type ArbitrationPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK_STATUS` reader - Not acknowledge error status bit. Error on I2C not\n\nacknowledge. Note: This bit field is read by the sticky\n\nbits present on the ih_i2cmphy_stat0 register."]
pub type NackStatusR = crate::BitReader;
#[doc = "Field `NACK_INTERRUPT` reader - Not acknowledge error interrupt bit. Only lasts for\n\none SFR clock cycle and is auto cleared after it.\n\n{nack_interrupt = (nack_mask==0b) &amp;&amp;\n\n(nack_status==nack_pol)}. Note: This bit field is\n\nread by the sticky bits present on the\n\nih_i2cmphy_stat0 register."]
pub type NackInterruptR = crate::BitReader;
#[doc = "Field `NACK_MASK` reader - Not acknowledge error interrupt mask signal"]
pub type NackMaskR = crate::BitReader;
#[doc = "Field `NACK_MASK` writer - Not acknowledge error interrupt mask signal"]
pub type NackMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK_POL` reader - Not acknowledge error interrupt polarity\n\nconfiguration"]
pub type NackPolR = crate::BitReader;
#[doc = "Field `NACK_POL` writer - Not acknowledge error interrupt polarity\n\nconfiguration"]
pub type NackPolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Arbitration error interrupt bit\n\n{arbitration_interrupt = (arbitration_mask==0b) &amp;&amp;\n\n(arbitration_status==arbitration_pol)}\n\nNote: This bit field is read by the sticky bits present\n\non the ih_i2cmphy_stat0 register."]
    #[inline(always)]
    pub fn arbitration_interrupt(&self) -> ArbitrationInterruptR {
        ArbitrationInterruptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Arbitration error interrupt mask signal."]
    #[inline(always)]
    pub fn arbitration_mask(&self) -> ArbitrationMaskR {
        ArbitrationMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Arbitration error interrupt polarity configuration."]
    #[inline(always)]
    pub fn arbitration_pol(&self) -> ArbitrationPolR {
        ArbitrationPolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge error status bit. Error on I2C not\n\nacknowledge. Note: This bit field is read by the sticky\n\nbits present on the ih_i2cmphy_stat0 register."]
    #[inline(always)]
    pub fn nack_status(&self) -> NackStatusR {
        NackStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Not acknowledge error interrupt bit. Only lasts for\n\none SFR clock cycle and is auto cleared after it.\n\n{nack_interrupt = (nack_mask==0b) &amp;&amp;\n\n(nack_status==nack_pol)}. Note: This bit field is\n\nread by the sticky bits present on the\n\nih_i2cmphy_stat0 register."]
    #[inline(always)]
    pub fn nack_interrupt(&self) -> NackInterruptR {
        NackInterruptR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Not acknowledge error interrupt mask signal"]
    #[inline(always)]
    pub fn nack_mask(&self) -> NackMaskR {
        NackMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Not acknowledge error interrupt polarity\n\nconfiguration"]
    #[inline(always)]
    pub fn nack_pol(&self) -> NackPolR {
        NackPolR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Arbitration error interrupt mask signal."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_mask(&mut self) -> ArbitrationMaskW<PhyI2cmCtlintSpec> {
        ArbitrationMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Arbitration error interrupt polarity configuration."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_pol(&mut self) -> ArbitrationPolW<PhyI2cmCtlintSpec> {
        ArbitrationPolW::new(self, 3)
    }
    #[doc = "Bit 6 - Not acknowledge error interrupt mask signal"]
    #[inline(always)]
    #[must_use]
    pub fn nack_mask(&mut self) -> NackMaskW<PhyI2cmCtlintSpec> {
        NackMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Not acknowledge error interrupt polarity\n\nconfiguration"]
    #[inline(always)]
    #[must_use]
    pub fn nack_pol(&mut self) -> NackPolW<PhyI2cmCtlintSpec> {
        NackPolW::new(self, 7)
    }
}
#[doc = "PHY I2C error Interrupt Register\n\nThis register contains and configures the I2C master PHY error interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_ctlint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_ctlint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmCtlintSpec;
impl crate::RegisterSpec for PhyI2cmCtlintSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_ctlint::R`](R) reader structure"]
impl crate::Readable for PhyI2cmCtlintSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_ctlint::W`](W) writer structure"]
impl crate::Writable for PhyI2cmCtlintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_CTLINT to value 0x88"]
impl crate::Resettable for PhyI2cmCtlintSpec {
    const RESET_VALUE: u8 = 0x88;
}
