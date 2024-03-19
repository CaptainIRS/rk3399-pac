#[doc = "Register `GMAC_AN_LINK_PART_AB` reader"]
pub type R = crate::R<GmacAnLinkPartAbSpec>;
#[doc = "Field `FD` reader - Full-Duplex\n\nWhen set, this bit indicates that the link partner has the ability to\n\noperate in Full-Duplex mode. When cleared, the link partner does\n\nnot have the ability to operate in Full-Duplex mode."]
pub type FdR = crate::BitReader;
#[doc = "Field `HD` reader - Half-Duplex\n\nWhen set, this bit indicates that the link partner has the ability to\n\noperate in Half-Duplex mode. When cleared, the link partner does\n\nnot have the ability to operate in Half-Duplex mode."]
pub type HdR = crate::BitReader;
#[doc = "Field `PSE` reader - Pause Encoding\n\nThese 2 bits provide an encoding for the PAUSE bits, indicating\n\nthat the link partner's capability of configuring the PAUSE\n\nfunction as defined in IEEE 802.3x."]
pub type PseR = crate::FieldReader;
#[doc = "Field `RFE` reader - Remote Fault Encoding\n\nThese 2 bits provide a remote fault encoding, indicating a fault or\n\nerror condition of the link partner."]
pub type RfeR = crate::FieldReader;
#[doc = "Field `ACK` reader - Acknowledge\n\nWhen set, this bit is used by the auto-negotiation function to\n\nindicate that the link partner has successfully received the\n\nGMAC's base page. When cleared, it indicates that a successful\n\nreceipt of the base page has not been achieved."]
pub type AckR = crate::BitReader;
#[doc = "Field `NP` reader - Next Page Support\n\nWhen set, this bit indicates that more next page information is\n\navailable.\n\nWhen cleared, this bit indicates that next page exchange is not\n\ndesired."]
pub type NpR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Full-Duplex\n\nWhen set, this bit indicates that the link partner has the ability to\n\noperate in Full-Duplex mode. When cleared, the link partner does\n\nnot have the ability to operate in Full-Duplex mode."]
    #[inline(always)]
    pub fn fd(&self) -> FdR {
        FdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Half-Duplex\n\nWhen set, this bit indicates that the link partner has the ability to\n\noperate in Half-Duplex mode. When cleared, the link partner does\n\nnot have the ability to operate in Half-Duplex mode."]
    #[inline(always)]
    pub fn hd(&self) -> HdR {
        HdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Pause Encoding\n\nThese 2 bits provide an encoding for the PAUSE bits, indicating\n\nthat the link partner's capability of configuring the PAUSE\n\nfunction as defined in IEEE 802.3x."]
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Remote Fault Encoding\n\nThese 2 bits provide a remote fault encoding, indicating a fault or\n\nerror condition of the link partner."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Acknowledge\n\nWhen set, this bit is used by the auto-negotiation function to\n\nindicate that the link partner has successfully received the\n\nGMAC's base page. When cleared, it indicates that a successful\n\nreceipt of the base page has not been achieved."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Next Page Support\n\nWhen set, this bit indicates that more next page information is\n\navailable.\n\nWhen cleared, this bit indicates that next page exchange is not\n\ndesired."]
    #[inline(always)]
    pub fn np(&self) -> NpR {
        NpR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Auto Negotiation Link Partner Ability Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_link_part_ab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacAnLinkPartAbSpec;
impl crate::RegisterSpec for GmacAnLinkPartAbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_an_link_part_ab::R`](R) reader structure"]
impl crate::Readable for GmacAnLinkPartAbSpec {}
#[doc = "`reset()` method sets GMAC_AN_LINK_PART_AB to value 0"]
impl crate::Resettable for GmacAnLinkPartAbSpec {
    const RESET_VALUE: u32 = 0;
}
