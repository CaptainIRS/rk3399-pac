#[doc = "Register `DENALI_PHY_913` reader"]
pub type R = crate::R<DenaliPhy913Spec>;
#[doc = "Register `DENALI_PHY_913` writer"]
pub type W = crate::W<DenaliPhy913Spec>;
#[doc = "Field `PHY_LOW_FREQ_SEL` reader - Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
pub type PhyLowFreqSelR = crate::BitReader;
#[doc = "Field `PHY_LOW_FREQ_SEL` writer - Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
pub type PhyLowFreqSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_0` reader - Pad VREF control settings for DQ slice 0."]
pub type PhyPadVrefCtrlDq0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_0` writer - Pad VREF control settings for DQ slice 0."]
pub type PhyPadVrefCtrlDq0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
    #[inline(always)]
    pub fn phy_low_freq_sel(&self) -> PhyLowFreqSelR {
        PhyLowFreqSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:19 - Pad VREF control settings for DQ slice 0."]
    #[inline(always)]
    pub fn phy_pad_vref_ctrl_dq_0(&self) -> PhyPadVrefCtrlDq0R {
        PhyPadVrefCtrlDq0R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_low_freq_sel(&mut self) -> PhyLowFreqSelW<DenaliPhy913Spec> {
        PhyLowFreqSelW::new(self, 0)
    }
    #[doc = "Bits 8:19 - Pad VREF control settings for DQ slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_vref_ctrl_dq_0(&mut self) -> PhyPadVrefCtrlDq0W<DenaliPhy913Spec> {
        PhyPadVrefCtrlDq0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_913::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_913::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy913Spec;
impl crate::RegisterSpec for DenaliPhy913Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_913::R`](R) reader structure"]
impl crate::Readable for DenaliPhy913Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_913::W`](W) writer structure"]
impl crate::Writable for DenaliPhy913Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_913 to value 0"]
impl crate::Resettable for DenaliPhy913Spec {
    const RESET_VALUE: u32 = 0;
}
