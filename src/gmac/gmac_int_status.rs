#[doc = "Register `GMAC_INT_STATUS` reader"]
pub type R = crate::R<GmacIntStatusSpec>;
#[doc = "Field `RIS` reader - RGMII Interrupt Status This bit is set due to any change in value of the Link Status of RGMII interface. This bit is cleared when the user makes a read operation the RGMII Status register."]
pub type RisR = crate::BitReader;
#[doc = "Field `PIS` reader - PMT Interrupt Status This bit is set whenever a Magic packet or Wake-on-LAN frame is received in Power-Down mode). This bit is cleared when both bits\\[6:5\\]
are cleared due to a read operation to the register GMAC_PMT_CTRL_STA."]
pub type PisR = crate::BitReader;
#[doc = "Field `MIS` reader - MMC Interrupt Status This bit is set high whenever any of bits 7:5 is set high and cleared only when all of these bits are low. This bit is valid only when the optional MMC module is selected during configuration."]
pub type MisR = crate::BitReader;
#[doc = "Field `MRIS` reader - MMC Receive Interrupt Status This bit is set high whenever an interrupt is generated in the MMC Receive Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared. This bit is only valid when the optional MMC module is selected during configuration."]
pub type MrisR = crate::BitReader;
#[doc = "Field `MTIS` reader - MMC Transmit Interrupt Status This bit is set high whenever an interrupt is generated in the MMC Transmit Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared. This bit is only valid when the optional MMC module is selected during configuration."]
pub type MtisR = crate::BitReader;
#[doc = "Field `MRCOIS` reader - MMC Receive Checksum Offload Interrupt Status This bit is set high whenever an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
pub type MrcoisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RGMII Interrupt Status This bit is set due to any change in value of the Link Status of RGMII interface. This bit is cleared when the user makes a read operation the RGMII Status register."]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Status This bit is set whenever a Magic packet or Wake-on-LAN frame is received in Power-Down mode). This bit is cleared when both bits\\[6:5\\]
are cleared due to a read operation to the register GMAC_PMT_CTRL_STA."]
    #[inline(always)]
    pub fn pis(&self) -> PisR {
        PisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status This bit is set high whenever any of bits 7:5 is set high and cleared only when all of these bits are low. This bit is valid only when the optional MMC module is selected during configuration."]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status This bit is set high whenever an interrupt is generated in the MMC Receive Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared. This bit is only valid when the optional MMC module is selected during configuration."]
    #[inline(always)]
    pub fn mris(&self) -> MrisR {
        MrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status This bit is set high whenever an interrupt is generated in the MMC Transmit Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared. This bit is only valid when the optional MMC module is selected during configuration."]
    #[inline(always)]
    pub fn mtis(&self) -> MtisR {
        MtisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status This bit is set high whenever an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
    #[inline(always)]
    pub fn mrcois(&self) -> MrcoisR {
        MrcoisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacIntStatusSpec;
impl crate::RegisterSpec for GmacIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_int_status::R`](R) reader structure"]
impl crate::Readable for GmacIntStatusSpec {}
#[doc = "`reset()` method sets GMAC_INT_STATUS to value 0"]
impl crate::Resettable for GmacIntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
