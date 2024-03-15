#[doc = "Register `GMAC_GMII_ADDR` reader"]
pub type R = crate::R<GmacGmiiAddrSpec>;
#[doc = "Register `GMAC_GMII_ADDR` writer"]
pub type W = crate::W<GmacGmiiAddrSpec>;
#[doc = "Field `GB` reader - GMII Busy This bit should read a logic 0 before writing to Register GMII_ADDR and Register GMII_DATA. This bit must also be set to 0 during a Write to Register GMII_ADDR. During a PHY register access, this bit will be set to 1'b1 by the Application to indicate that a Read or Write access is in progress. Register GMII_DATA (GMII Data) should be kept valid until this bit is cleared by the GMAC during a PHY Write operation. The Register GMII_DATA is invalid until this bit is cleared by the GMAC during a PHY Read operation. The Register GMII_ADDR (GMII Address) should not be written to until this bit is cleared."]
pub type GbR = crate::BitReader;
#[doc = "Field `GB` writer - GMII Busy This bit should read a logic 0 before writing to Register GMII_ADDR and Register GMII_DATA. This bit must also be set to 0 during a Write to Register GMII_ADDR. During a PHY register access, this bit will be set to 1'b1 by the Application to indicate that a Read or Write access is in progress. Register GMII_DATA (GMII Data) should be kept valid until this bit is cleared by the GMAC during a PHY Write operation. The Register GMII_DATA is invalid until this bit is cleared by the GMAC during a PHY Read operation. The Register GMII_ADDR (GMII Address) should not be written to until this bit is cleared."]
pub type GbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GW` reader - GMII Write When set, this bit tells the PHY that this will be a Write operation using register GMAC_GMII_DATA. If this bit is not set, this will be a Read operation, placing the data in register GMAC_GMII_DATA."]
pub type GwR = crate::BitReader;
#[doc = "Field `GW` writer - GMII Write When set, this bit tells the PHY that this will be a Write operation using register GMAC_GMII_DATA. If this bit is not set, this will be a Read operation, placing the data in register GMAC_GMII_DATA."]
pub type GwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - APB Clock Range The APB Clock Range selection determines the frequency of the MDC clock as per the pclk_gmac frequency used in your design. The suggested range of pclk_gmac frequency applicable for each value below (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz - 2.5 MHz. Selection pclk_gmacMDC Clock 0000 60-100 MHz pclk_gmac/42 0001 100-150 MHz pclk_gmac/62 0010 20-35 MHz pclk_gmac/16 0011 35-60 MHz pclk_gmac/26 0100 150-250 MHz pclk_gmac/102 0101 250-300 MHz pclk_gmac/124 0110, 0111 Reserved When bit 5 is set, you can achieve MDC clock of frequency higher than the IEEE802.3 specified frequency limit of 2.5 MHz and program a clock divider of lower value. For example, when pclk_gmac is of frequency 100 MHz and you program these bits as \"1010\", then the resultant MDC clock will be of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Please program the values given below only if the interfacing chips supports faster MDC clocks. Selection MDC Clock 1000 pclk_gmac/4 1001 pclk_gmac/6 1010 pclk_gmac/8 1011 pclk_gmac/10 1100 pclk_gmac/12 1101 pclk_gmac/14 1110 pclk_gmac/16 1111 pclk_gmac/18"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - APB Clock Range The APB Clock Range selection determines the frequency of the MDC clock as per the pclk_gmac frequency used in your design. The suggested range of pclk_gmac frequency applicable for each value below (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz - 2.5 MHz. Selection pclk_gmacMDC Clock 0000 60-100 MHz pclk_gmac/42 0001 100-150 MHz pclk_gmac/62 0010 20-35 MHz pclk_gmac/16 0011 35-60 MHz pclk_gmac/26 0100 150-250 MHz pclk_gmac/102 0101 250-300 MHz pclk_gmac/124 0110, 0111 Reserved When bit 5 is set, you can achieve MDC clock of frequency higher than the IEEE802.3 specified frequency limit of 2.5 MHz and program a clock divider of lower value. For example, when pclk_gmac is of frequency 100 MHz and you program these bits as \"1010\", then the resultant MDC clock will be of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Please program the values given below only if the interfacing chips supports faster MDC clocks. Selection MDC Clock 1000 pclk_gmac/4 1001 pclk_gmac/6 1010 pclk_gmac/8 1011 pclk_gmac/10 1100 pclk_gmac/12 1101 pclk_gmac/14 1110 pclk_gmac/16 1111 pclk_gmac/18"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GR` reader - GMII Register These bits select the desired GMII register in the selected PHY device"]
pub type GrR = crate::FieldReader;
#[doc = "Field `GR` writer - GMII Register These bits select the desired GMII register in the selected PHY device"]
pub type GrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address This field tells which of the 32 possible PHY devices are being accessed"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address This field tells which of the 32 possible PHY devices are being accessed"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - GMII Busy This bit should read a logic 0 before writing to Register GMII_ADDR and Register GMII_DATA. This bit must also be set to 0 during a Write to Register GMII_ADDR. During a PHY register access, this bit will be set to 1'b1 by the Application to indicate that a Read or Write access is in progress. Register GMII_DATA (GMII Data) should be kept valid until this bit is cleared by the GMAC during a PHY Write operation. The Register GMII_DATA is invalid until this bit is cleared by the GMAC during a PHY Read operation. The Register GMII_ADDR (GMII Address) should not be written to until this bit is cleared."]
    #[inline(always)]
    pub fn gb(&self) -> GbR {
        GbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMII Write When set, this bit tells the PHY that this will be a Write operation using register GMAC_GMII_DATA. If this bit is not set, this will be a Read operation, placing the data in register GMAC_GMII_DATA."]
    #[inline(always)]
    pub fn gw(&self) -> GwR {
        GwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - APB Clock Range The APB Clock Range selection determines the frequency of the MDC clock as per the pclk_gmac frequency used in your design. The suggested range of pclk_gmac frequency applicable for each value below (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz - 2.5 MHz. Selection pclk_gmacMDC Clock 0000 60-100 MHz pclk_gmac/42 0001 100-150 MHz pclk_gmac/62 0010 20-35 MHz pclk_gmac/16 0011 35-60 MHz pclk_gmac/26 0100 150-250 MHz pclk_gmac/102 0101 250-300 MHz pclk_gmac/124 0110, 0111 Reserved When bit 5 is set, you can achieve MDC clock of frequency higher than the IEEE802.3 specified frequency limit of 2.5 MHz and program a clock divider of lower value. For example, when pclk_gmac is of frequency 100 MHz and you program these bits as \"1010\", then the resultant MDC clock will be of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Please program the values given below only if the interfacing chips supports faster MDC clocks. Selection MDC Clock 1000 pclk_gmac/4 1001 pclk_gmac/6 1010 pclk_gmac/8 1011 pclk_gmac/10 1100 pclk_gmac/12 1101 pclk_gmac/14 1110 pclk_gmac/16 1111 pclk_gmac/18"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - GMII Register These bits select the desired GMII register in the selected PHY device"]
    #[inline(always)]
    pub fn gr(&self) -> GrR {
        GrR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address This field tells which of the 32 possible PHY devices are being accessed"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GMII Busy This bit should read a logic 0 before writing to Register GMII_ADDR and Register GMII_DATA. This bit must also be set to 0 during a Write to Register GMII_ADDR. During a PHY register access, this bit will be set to 1'b1 by the Application to indicate that a Read or Write access is in progress. Register GMII_DATA (GMII Data) should be kept valid until this bit is cleared by the GMAC during a PHY Write operation. The Register GMII_DATA is invalid until this bit is cleared by the GMAC during a PHY Read operation. The Register GMII_ADDR (GMII Address) should not be written to until this bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn gb(&mut self) -> GbW<GmacGmiiAddrSpec> {
        GbW::new(self, 0)
    }
    #[doc = "Bit 1 - GMII Write When set, this bit tells the PHY that this will be a Write operation using register GMAC_GMII_DATA. If this bit is not set, this will be a Read operation, placing the data in register GMAC_GMII_DATA."]
    #[inline(always)]
    #[must_use]
    pub fn gw(&mut self) -> GwW<GmacGmiiAddrSpec> {
        GwW::new(self, 1)
    }
    #[doc = "Bits 2:5 - APB Clock Range The APB Clock Range selection determines the frequency of the MDC clock as per the pclk_gmac frequency used in your design. The suggested range of pclk_gmac frequency applicable for each value below (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz - 2.5 MHz. Selection pclk_gmacMDC Clock 0000 60-100 MHz pclk_gmac/42 0001 100-150 MHz pclk_gmac/62 0010 20-35 MHz pclk_gmac/16 0011 35-60 MHz pclk_gmac/26 0100 150-250 MHz pclk_gmac/102 0101 250-300 MHz pclk_gmac/124 0110, 0111 Reserved When bit 5 is set, you can achieve MDC clock of frequency higher than the IEEE802.3 specified frequency limit of 2.5 MHz and program a clock divider of lower value. For example, when pclk_gmac is of frequency 100 MHz and you program these bits as \"1010\", then the resultant MDC clock will be of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Please program the values given below only if the interfacing chips supports faster MDC clocks. Selection MDC Clock 1000 pclk_gmac/4 1001 pclk_gmac/6 1010 pclk_gmac/8 1011 pclk_gmac/10 1100 pclk_gmac/12 1101 pclk_gmac/14 1110 pclk_gmac/16 1111 pclk_gmac/18"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<GmacGmiiAddrSpec> {
        CrW::new(self, 2)
    }
    #[doc = "Bits 6:10 - GMII Register These bits select the desired GMII register in the selected PHY device"]
    #[inline(always)]
    #[must_use]
    pub fn gr(&mut self) -> GrW<GmacGmiiAddrSpec> {
        GrW::new(self, 6)
    }
    #[doc = "Bits 11:15 - Physical Layer Address This field tells which of the 32 possible PHY devices are being accessed"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PaW<GmacGmiiAddrSpec> {
        PaW::new(self, 11)
    }
}
#[doc = "GMII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_gmii_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_gmii_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacGmiiAddrSpec;
impl crate::RegisterSpec for GmacGmiiAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_gmii_addr::R`](R) reader structure"]
impl crate::Readable for GmacGmiiAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_gmii_addr::W`](W) writer structure"]
impl crate::Writable for GmacGmiiAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets GMAC_GMII_ADDR to value 0"]
impl crate::Resettable for GmacGmiiAddrSpec {
    const RESET_VALUE: u32 = 0;
}
