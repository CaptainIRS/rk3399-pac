#[doc = "Register `GMAC_AN_ADV` reader"]
pub type R = crate::R<GmacAnAdvSpec>;
#[doc = "Register `GMAC_AN_ADV` writer"]
pub type W = crate::W<GmacAnAdvSpec>;
#[doc = "Field `FD` reader - Full-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Full-\n\nDuplex."]
pub type FdR = crate::BitReader;
#[doc = "Field `FD` writer - Full-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Full-\n\nDuplex."]
pub type FdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HD` reader - Half-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Half-\n\nDuplex. This bit is tied to low (and RO) when the GMAC is\n\nconfigured for Full-Duplex-only operation."]
pub type HdR = crate::BitReader;
#[doc = "Field `HD` writer - Half-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Half-\n\nDuplex. This bit is tied to low (and RO) when the GMAC is\n\nconfigured for Full-Duplex-only operation."]
pub type HdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSE` reader - Pause Encoding\n\nThese 2 bits provide an encoding for the PAUSE bits, indicating\n\nthat the GMAC is capable of configuring the PAUSE function as\n\ndefined in IEEE 802.3x."]
pub type PseR = crate::FieldReader;
#[doc = "Field `PSE` writer - Pause Encoding\n\nThese 2 bits provide an encoding for the PAUSE bits, indicating\n\nthat the GMAC is capable of configuring the PAUSE function as\n\ndefined in IEEE 802.3x."]
pub type PseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFE` reader - Remote Fault Encoding\n\nThese 2 bits provide a remote fault encoding, indicating to a link\n\npartner that a fault or error condition has occurred."]
pub type RfeR = crate::FieldReader;
#[doc = "Field `RFE` writer - Remote Fault Encoding\n\nThese 2 bits provide a remote fault encoding, indicating to a link\n\npartner that a fault or error condition has occurred."]
pub type RfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NP` reader - Next Page Support\n\nThis bit is tied to low, because the GMAC does not support the\n\nnext page."]
pub type NpR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Full-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Full-\n\nDuplex."]
    #[inline(always)]
    pub fn fd(&self) -> FdR {
        FdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Half-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Half-\n\nDuplex. This bit is tied to low (and RO) when the GMAC is\n\nconfigured for Full-Duplex-only operation."]
    #[inline(always)]
    pub fn hd(&self) -> HdR {
        HdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Pause Encoding\n\nThese 2 bits provide an encoding for the PAUSE bits, indicating\n\nthat the GMAC is capable of configuring the PAUSE function as\n\ndefined in IEEE 802.3x."]
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Remote Fault Encoding\n\nThese 2 bits provide a remote fault encoding, indicating to a link\n\npartner that a fault or error condition has occurred."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Next Page Support\n\nThis bit is tied to low, because the GMAC does not support the\n\nnext page."]
    #[inline(always)]
    pub fn np(&self) -> NpR {
        NpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Full-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Full-\n\nDuplex."]
    #[inline(always)]
    #[must_use]
    pub fn fd(&mut self) -> FdW<GmacAnAdvSpec> {
        FdW::new(self, 5)
    }
    #[doc = "Bit 6 - Half-Duplex\n\nThis bit, when set high, indicates that the GMAC supports Half-\n\nDuplex. This bit is tied to low (and RO) when the GMAC is\n\nconfigured for Full-Duplex-only operation."]
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HdW<GmacAnAdvSpec> {
        HdW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Pause Encoding\n\nThese 2 bits provide an encoding for the PAUSE bits, indicating\n\nthat the GMAC is capable of configuring the PAUSE function as\n\ndefined in IEEE 802.3x."]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PseW<GmacAnAdvSpec> {
        PseW::new(self, 7)
    }
    #[doc = "Bits 12:13 - Remote Fault Encoding\n\nThese 2 bits provide a remote fault encoding, indicating to a link\n\npartner that a fault or error condition has occurred."]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<GmacAnAdvSpec> {
        RfeW::new(self, 12)
    }
}
#[doc = "Auto Negotiation Advertisement Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_adv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_an_adv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacAnAdvSpec;
impl crate::RegisterSpec for GmacAnAdvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_an_adv::R`](R) reader structure"]
impl crate::Readable for GmacAnAdvSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_an_adv::W`](W) writer structure"]
impl crate::Writable for GmacAnAdvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_AN_ADV to value 0x01e0"]
impl crate::Resettable for GmacAnAdvSpec {
    const RESET_VALUE: u32 = 0x01e0;
}
