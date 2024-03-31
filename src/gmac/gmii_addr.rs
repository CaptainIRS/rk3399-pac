#[doc = "Register `GMII_ADDR` reader"]
pub type R = crate::R<GmiiAddrSpec>;
#[doc = "Register `GMII_ADDR` writer"]
pub type W = crate::W<GmiiAddrSpec>;
#[doc = "Field `GB` reader - GMII Busy\n\nThis bit should read a logic 0 before writing to Register\n\nGMII_ADDR and Register GMII_DATA. This bit must also be set to\n\n0 during a Write to Register GMII_ADDR. During a PHY register\n\naccess, this bit will be set to 1'b1 by the Application to indicate\n\nthat a Read or Write access is in progress. Register GMII_DATA\n\n(GMII Data) should be kept valid until this bit is cleared by the\n\nGMAC during a PHY Write operation. The Register GMII_DATA is\n\ninvalid until this bit is cleared by the GMAC during a PHY Read\n\noperation. The Register GMII_ADDR (GMII Address) should not\n\nbe written to until this bit is cleared."]
pub type GbR = crate::BitReader;
#[doc = "Field `GB` writer - GMII Busy\n\nThis bit should read a logic 0 before writing to Register\n\nGMII_ADDR and Register GMII_DATA. This bit must also be set to\n\n0 during a Write to Register GMII_ADDR. During a PHY register\n\naccess, this bit will be set to 1'b1 by the Application to indicate\n\nthat a Read or Write access is in progress. Register GMII_DATA\n\n(GMII Data) should be kept valid until this bit is cleared by the\n\nGMAC during a PHY Write operation. The Register GMII_DATA is\n\ninvalid until this bit is cleared by the GMAC during a PHY Read\n\noperation. The Register GMII_ADDR (GMII Address) should not\n\nbe written to until this bit is cleared."]
pub type GbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GW` reader - GMII Write\n\nWhen set, this bit tells the PHY that this will be a Write operation\n\nusing register GMAC_GMII_DATA. If this bit is not set, this will be\n\na Read operation, placing the data in register GMAC_GMII_DATA."]
pub type GwR = crate::BitReader;
#[doc = "Field `GW` writer - GMII Write\n\nWhen set, this bit tells the PHY that this will be a Write operation\n\nusing register GMAC_GMII_DATA. If this bit is not set, this will be\n\na Read operation, placing the data in register GMAC_GMII_DATA."]
pub type GwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - APB Clock Range\n\nThe APB Clock Range selection determines the frequency of the\n\nMDC clock as per the pclk_gmac frequency used in your design.\n\nThe suggested range of pclk_gmac frequency applicable for each\n\nvalue below (when Bit\\[5\\]
= 0) ensures that the MDC clock is\n\napproximately between the frequency range 1.0 MHz - 2.5 MHz.\n\nSelection pclk_gmacMDC Clock\n\n0000 60-100 MHz pclk_gmac/42\n\n0001 100-150 MHz pclk_gmac/62\n\n0010 20-35 MHz pclk_gmac/16\n\n0011 35-60 MHz pclk_gmac/26\n\n0100 150-250 MHz pclk_gmac/102\n\n0101 250-300 MHz pclk_gmac/124\n\n0110, 0111 Reserved\n\nWhen bit 5 is set, you can achieve MDC clock of frequency higher\n\nthan the IEEE802.3 specified frequency limit of 2.5 MHz and\n\nprogram a clock divider of lower value. For example, when\n\npclk_gmac is of frequency 100 MHz and you program these bits\n\nas '1010', then the resultant MDC clock will be of 12.5 MHz\n\nwhich is outside the limit of IEEE 802.3 specified range. Please\n\nprogram the values given below only if the interfacing chips\n\nsupports faster MDC clocks.\n\nSelection MDC Clock\n\n1000 pclk_gmac/4\n\n1001 pclk_gmac/6\n\n1010 pclk_gmac/8\n\n1011 pclk_gmac/10\n\n1100 pclk_gmac/12\n\n1101 pclk_gmac/14\n\n1110 pclk_gmac/16\n\n1111 pclk_gmac/18"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - APB Clock Range\n\nThe APB Clock Range selection determines the frequency of the\n\nMDC clock as per the pclk_gmac frequency used in your design.\n\nThe suggested range of pclk_gmac frequency applicable for each\n\nvalue below (when Bit\\[5\\]
= 0) ensures that the MDC clock is\n\napproximately between the frequency range 1.0 MHz - 2.5 MHz.\n\nSelection pclk_gmacMDC Clock\n\n0000 60-100 MHz pclk_gmac/42\n\n0001 100-150 MHz pclk_gmac/62\n\n0010 20-35 MHz pclk_gmac/16\n\n0011 35-60 MHz pclk_gmac/26\n\n0100 150-250 MHz pclk_gmac/102\n\n0101 250-300 MHz pclk_gmac/124\n\n0110, 0111 Reserved\n\nWhen bit 5 is set, you can achieve MDC clock of frequency higher\n\nthan the IEEE802.3 specified frequency limit of 2.5 MHz and\n\nprogram a clock divider of lower value. For example, when\n\npclk_gmac is of frequency 100 MHz and you program these bits\n\nas '1010', then the resultant MDC clock will be of 12.5 MHz\n\nwhich is outside the limit of IEEE 802.3 specified range. Please\n\nprogram the values given below only if the interfacing chips\n\nsupports faster MDC clocks.\n\nSelection MDC Clock\n\n1000 pclk_gmac/4\n\n1001 pclk_gmac/6\n\n1010 pclk_gmac/8\n\n1011 pclk_gmac/10\n\n1100 pclk_gmac/12\n\n1101 pclk_gmac/14\n\n1110 pclk_gmac/16\n\n1111 pclk_gmac/18"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GR` reader - GMII Register\n\nThese bits select the desired GMII register in the selected PHY\n\ndevice"]
pub type GrR = crate::FieldReader;
#[doc = "Field `GR` writer - GMII Register\n\nThese bits select the desired GMII register in the selected PHY\n\ndevice"]
pub type GrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address\n\nThis field tells which of the 32 possible PHY devices are being\n\naccessed"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address\n\nThis field tells which of the 32 possible PHY devices are being\n\naccessed"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - GMII Busy\n\nThis bit should read a logic 0 before writing to Register\n\nGMII_ADDR and Register GMII_DATA. This bit must also be set to\n\n0 during a Write to Register GMII_ADDR. During a PHY register\n\naccess, this bit will be set to 1'b1 by the Application to indicate\n\nthat a Read or Write access is in progress. Register GMII_DATA\n\n(GMII Data) should be kept valid until this bit is cleared by the\n\nGMAC during a PHY Write operation. The Register GMII_DATA is\n\ninvalid until this bit is cleared by the GMAC during a PHY Read\n\noperation. The Register GMII_ADDR (GMII Address) should not\n\nbe written to until this bit is cleared."]
    #[inline(always)]
    pub fn gb(&self) -> GbR {
        GbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMII Write\n\nWhen set, this bit tells the PHY that this will be a Write operation\n\nusing register GMAC_GMII_DATA. If this bit is not set, this will be\n\na Read operation, placing the data in register GMAC_GMII_DATA."]
    #[inline(always)]
    pub fn gw(&self) -> GwR {
        GwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - APB Clock Range\n\nThe APB Clock Range selection determines the frequency of the\n\nMDC clock as per the pclk_gmac frequency used in your design.\n\nThe suggested range of pclk_gmac frequency applicable for each\n\nvalue below (when Bit\\[5\\]
= 0) ensures that the MDC clock is\n\napproximately between the frequency range 1.0 MHz - 2.5 MHz.\n\nSelection pclk_gmacMDC Clock\n\n0000 60-100 MHz pclk_gmac/42\n\n0001 100-150 MHz pclk_gmac/62\n\n0010 20-35 MHz pclk_gmac/16\n\n0011 35-60 MHz pclk_gmac/26\n\n0100 150-250 MHz pclk_gmac/102\n\n0101 250-300 MHz pclk_gmac/124\n\n0110, 0111 Reserved\n\nWhen bit 5 is set, you can achieve MDC clock of frequency higher\n\nthan the IEEE802.3 specified frequency limit of 2.5 MHz and\n\nprogram a clock divider of lower value. For example, when\n\npclk_gmac is of frequency 100 MHz and you program these bits\n\nas '1010', then the resultant MDC clock will be of 12.5 MHz\n\nwhich is outside the limit of IEEE 802.3 specified range. Please\n\nprogram the values given below only if the interfacing chips\n\nsupports faster MDC clocks.\n\nSelection MDC Clock\n\n1000 pclk_gmac/4\n\n1001 pclk_gmac/6\n\n1010 pclk_gmac/8\n\n1011 pclk_gmac/10\n\n1100 pclk_gmac/12\n\n1101 pclk_gmac/14\n\n1110 pclk_gmac/16\n\n1111 pclk_gmac/18"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - GMII Register\n\nThese bits select the desired GMII register in the selected PHY\n\ndevice"]
    #[inline(always)]
    pub fn gr(&self) -> GrR {
        GrR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address\n\nThis field tells which of the 32 possible PHY devices are being\n\naccessed"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GMII Busy\n\nThis bit should read a logic 0 before writing to Register\n\nGMII_ADDR and Register GMII_DATA. This bit must also be set to\n\n0 during a Write to Register GMII_ADDR. During a PHY register\n\naccess, this bit will be set to 1'b1 by the Application to indicate\n\nthat a Read or Write access is in progress. Register GMII_DATA\n\n(GMII Data) should be kept valid until this bit is cleared by the\n\nGMAC during a PHY Write operation. The Register GMII_DATA is\n\ninvalid until this bit is cleared by the GMAC during a PHY Read\n\noperation. The Register GMII_ADDR (GMII Address) should not\n\nbe written to until this bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn gb(&mut self) -> GbW<GmiiAddrSpec> {
        GbW::new(self, 0)
    }
    #[doc = "Bit 1 - GMII Write\n\nWhen set, this bit tells the PHY that this will be a Write operation\n\nusing register GMAC_GMII_DATA. If this bit is not set, this will be\n\na Read operation, placing the data in register GMAC_GMII_DATA."]
    #[inline(always)]
    #[must_use]
    pub fn gw(&mut self) -> GwW<GmiiAddrSpec> {
        GwW::new(self, 1)
    }
    #[doc = "Bits 2:5 - APB Clock Range\n\nThe APB Clock Range selection determines the frequency of the\n\nMDC clock as per the pclk_gmac frequency used in your design.\n\nThe suggested range of pclk_gmac frequency applicable for each\n\nvalue below (when Bit\\[5\\]
= 0) ensures that the MDC clock is\n\napproximately between the frequency range 1.0 MHz - 2.5 MHz.\n\nSelection pclk_gmacMDC Clock\n\n0000 60-100 MHz pclk_gmac/42\n\n0001 100-150 MHz pclk_gmac/62\n\n0010 20-35 MHz pclk_gmac/16\n\n0011 35-60 MHz pclk_gmac/26\n\n0100 150-250 MHz pclk_gmac/102\n\n0101 250-300 MHz pclk_gmac/124\n\n0110, 0111 Reserved\n\nWhen bit 5 is set, you can achieve MDC clock of frequency higher\n\nthan the IEEE802.3 specified frequency limit of 2.5 MHz and\n\nprogram a clock divider of lower value. For example, when\n\npclk_gmac is of frequency 100 MHz and you program these bits\n\nas '1010', then the resultant MDC clock will be of 12.5 MHz\n\nwhich is outside the limit of IEEE 802.3 specified range. Please\n\nprogram the values given below only if the interfacing chips\n\nsupports faster MDC clocks.\n\nSelection MDC Clock\n\n1000 pclk_gmac/4\n\n1001 pclk_gmac/6\n\n1010 pclk_gmac/8\n\n1011 pclk_gmac/10\n\n1100 pclk_gmac/12\n\n1101 pclk_gmac/14\n\n1110 pclk_gmac/16\n\n1111 pclk_gmac/18"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<GmiiAddrSpec> {
        CrW::new(self, 2)
    }
    #[doc = "Bits 6:10 - GMII Register\n\nThese bits select the desired GMII register in the selected PHY\n\ndevice"]
    #[inline(always)]
    #[must_use]
    pub fn gr(&mut self) -> GrW<GmiiAddrSpec> {
        GrW::new(self, 6)
    }
    #[doc = "Bits 11:15 - Physical Layer Address\n\nThis field tells which of the 32 possible PHY devices are being\n\naccessed"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PaW<GmiiAddrSpec> {
        PaW::new(self, 11)
    }
}
#[doc = "GMII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmiiAddrSpec;
impl crate::RegisterSpec for GmiiAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmii_addr::R`](R) reader structure"]
impl crate::Readable for GmiiAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`gmii_addr::W`](W) writer structure"]
impl crate::Writable for GmiiAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets GMII_ADDR to value 0"]
impl crate::Resettable for GmiiAddrSpec {
    const RESET_VALUE: u32 = 0;
}
