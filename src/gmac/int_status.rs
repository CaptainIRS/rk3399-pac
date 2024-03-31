#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Field `RIS` reader - RGMII Interrupt Status\n\nThis bit is set due to any change in value of the Link Status of\n\nRGMII interface. This bit is cleared when the user makes a read\n\noperation the RGMII Status register."]
pub type RisR = crate::BitReader;
#[doc = "Field `PIS` reader - PMT Interrupt Status\n\nThis bit is set whenever a Magic packet or Wake-on-LAN frame is\n\nreceived in Power-Down mode). This bit is cleared when both\n\nbits\\[6:5\\]
are cleared due to a read operation to the register\n\nGMAC_PMT_CTRL_STA."]
pub type PisR = crate::BitReader;
#[doc = "Field `MIS` reader - MMC Interrupt Status\n\nThis bit is set high whenever any of bits 7:5 is set high and\n\ncleared only when all of these bits are low. This bit is valid only\n\nwhen the optional MMC module is selected during configuration."]
pub type MisR = crate::BitReader;
#[doc = "Field `MRIS` reader - MMC Receive Interrupt Status\n\nThis bit is set high whenever an interrupt is generated in the MMC\n\nReceive Interrupt Register. This bit is cleared when all the bits in\n\nthis interrupt register are cleared. This bit is only valid when the\n\noptional MMC module is selected during configuration."]
pub type MrisR = crate::BitReader;
#[doc = "Field `MTIS` reader - MMC Transmit Interrupt Status\n\nThis bit is set high whenever an interrupt is generated in the MMC\n\nTransmit Interrupt Register. This bit is cleared when all the bits in\n\nthis interrupt register are cleared. This bit is only valid when the\n\noptional MMC module is selected during configuration."]
pub type MtisR = crate::BitReader;
#[doc = "Field `MRCOIS` reader - MMC Receive Checksum Offload Interrupt Status\n\nThis bit is set high whenever an interrupt is generated in the MMC\n\nReceive Checksum Offload Interrupt Register. This bit is cleared\n\nwhen all the bits in this interrupt register are cleared."]
pub type MrcoisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RGMII Interrupt Status\n\nThis bit is set due to any change in value of the Link Status of\n\nRGMII interface. This bit is cleared when the user makes a read\n\noperation the RGMII Status register."]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Status\n\nThis bit is set whenever a Magic packet or Wake-on-LAN frame is\n\nreceived in Power-Down mode). This bit is cleared when both\n\nbits\\[6:5\\]
are cleared due to a read operation to the register\n\nGMAC_PMT_CTRL_STA."]
    #[inline(always)]
    pub fn pis(&self) -> PisR {
        PisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status\n\nThis bit is set high whenever any of bits 7:5 is set high and\n\ncleared only when all of these bits are low. This bit is valid only\n\nwhen the optional MMC module is selected during configuration."]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status\n\nThis bit is set high whenever an interrupt is generated in the MMC\n\nReceive Interrupt Register. This bit is cleared when all the bits in\n\nthis interrupt register are cleared. This bit is only valid when the\n\noptional MMC module is selected during configuration."]
    #[inline(always)]
    pub fn mris(&self) -> MrisR {
        MrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status\n\nThis bit is set high whenever an interrupt is generated in the MMC\n\nTransmit Interrupt Register. This bit is cleared when all the bits in\n\nthis interrupt register are cleared. This bit is only valid when the\n\noptional MMC module is selected during configuration."]
    #[inline(always)]
    pub fn mtis(&self) -> MtisR {
        MtisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status\n\nThis bit is set high whenever an interrupt is generated in the MMC\n\nReceive Checksum Offload Interrupt Register. This bit is cleared\n\nwhen all the bits in this interrupt register are cleared."]
    #[inline(always)]
    pub fn mrcois(&self) -> MrcoisR {
        MrcoisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
