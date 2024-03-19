#[doc = "Register `GMAC_MAC_FRM_FILT` reader"]
pub type R = crate::R<GmacMacFrmFiltSpec>;
#[doc = "Register `GMAC_MAC_FRM_FILT` writer"]
pub type W = crate::W<GmacMacFrmFiltSpec>;
#[doc = "Field `PR` reader - Promiscuous Mode\n\nWhen this bit is set, the Address Filter module passes all\n\nincoming frames regardless of its destination or source address.\n\nThe SA/DA Filter Fails status bits of the Receive Status Word will\n\nalways be cleared when PR is set."]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous Mode\n\nWhen this bit is set, the Address Filter module passes all\n\nincoming frames regardless of its destination or source address.\n\nThe SA/DA Filter Fails status bits of the Receive Status Word will\n\nalways be cleared when PR is set."]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - Hash Unicast\n\nWhen set, MAC performs destination address filtering of unicast\n\nframes according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for unicast frames, that is, it compares the DA field with\n\nthe values programmed in DA registers."]
pub type HucR = crate::BitReader;
#[doc = "Field `HUC` writer - Hash Unicast\n\nWhen set, MAC performs destination address filtering of unicast\n\nframes according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for unicast frames, that is, it compares the DA field with\n\nthe values programmed in DA registers."]
pub type HucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - Hash Multicast\n\nWhen set, MAC performs destination address filtering of received\n\nmulticast frames according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for multicast frames, that is, it compares the DA field\n\nwith the values programmed in DA registers."]
pub type HmcR = crate::BitReader;
#[doc = "Field `HMC` writer - Hash Multicast\n\nWhen set, MAC performs destination address filtering of received\n\nmulticast frames according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for multicast frames, that is, it compares the DA field\n\nwith the values programmed in DA registers."]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the DA address comparison for both unicast\n\nand multicast frames.\n\nWhen reset, normal filtering of frames is performed."]
pub type DaifR = crate::BitReader;
#[doc = "Field `DAIF` writer - DA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the DA address comparison for both unicast\n\nand multicast frames.\n\nWhen reset, normal filtering of frames is performed."]
pub type DaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Pass All Multicast\n\nWhen set, this bit indicates that all received frames with a\n\nmulticast destination address (first bit in the destination address\n\nfield is '1') are passed.\n\nWhen reset, filtering of multicast frame depends on HMC bit."]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - Pass All Multicast\n\nWhen set, this bit indicates that all received frames with a\n\nmulticast destination address (first bit in the destination address\n\nfield is '1') are passed.\n\nWhen reset, filtering of multicast frame depends on HMC bit."]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - Disable Broadcast Frames\n\nWhen this bit is set, the AFM module filters all incoming\n\nbroadcast frames.\n\nWhen this bit is reset, the AFM module passes all received\n\nbroadcast frames."]
pub type DbfR = crate::BitReader;
#[doc = "Field `DBF` writer - Disable Broadcast Frames\n\nWhen this bit is set, the AFM module filters all incoming\n\nbroadcast frames.\n\nWhen this bit is reset, the AFM module passes all received\n\nbroadcast frames."]
pub type DbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pass Control Frames\n\nThese bits control the forwarding of all control frames (including\n\nunicast and multicast PAUSE frames). Note that the processing of\n\nPAUSE control frames depends only on RFE of Register\n\nGMAC_FLOW_CTRL\\[2\\].\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcf {
    #[doc = "0: GMAC filters all control frames from reaching the application."]
    B00 = 0,
    #[doc = "1: GMAC forwards all control frames except PAUSE control frames to application even if they fail the Address filter."]
    B01 = 1,
    #[doc = "2: GMAC forwards all control frames to application even if they fail the Address Filter."]
    B10 = 2,
    #[doc = "3: GMAC forwards control frames that pass the Address Filter."]
    B11 = 3,
}
impl From<Pcf> for u8 {
    #[inline(always)]
    fn from(variant: Pcf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcf {
    type Ux = u8;
}
#[doc = "Field `PCF` reader - Pass Control Frames\n\nThese bits control the forwarding of all control frames (including\n\nunicast and multicast PAUSE frames). Note that the processing of\n\nPAUSE control frames depends only on RFE of Register\n\nGMAC_FLOW_CTRL\\[2\\]."]
pub type PcfR = crate::FieldReader<Pcf>;
impl PcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcf {
        match self.bits {
            0 => Pcf::B00,
            1 => Pcf::B01,
            2 => Pcf::B10,
            3 => Pcf::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "GMAC filters all control frames from reaching the application."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Pcf::B00
    }
    #[doc = "GMAC forwards all control frames except PAUSE control frames to application even if they fail the Address filter."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Pcf::B01
    }
    #[doc = "GMAC forwards all control frames to application even if they fail the Address Filter."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Pcf::B10
    }
    #[doc = "GMAC forwards control frames that pass the Address Filter."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Pcf::B11
    }
}
#[doc = "Field `PCF` writer - Pass Control Frames\n\nThese bits control the forwarding of all control frames (including\n\nunicast and multicast PAUSE frames). Note that the processing of\n\nPAUSE control frames depends only on RFE of Register\n\nGMAC_FLOW_CTRL\\[2\\]."]
pub type PcfW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Pcf>;
impl<'a, REG> PcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GMAC filters all control frames from reaching the application."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Pcf::B00)
    }
    #[doc = "GMAC forwards all control frames except PAUSE control frames to application even if they fail the Address filter."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Pcf::B01)
    }
    #[doc = "GMAC forwards all control frames to application even if they fail the Address Filter."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Pcf::B10)
    }
    #[doc = "GMAC forwards control frames that pass the Address Filter."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Pcf::B11)
    }
}
#[doc = "Field `SAIF` reader - SA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the SA address comparison. The frames whose\n\nSA matches the SA registers will be marked as failing the SA\n\nAddress filter.\n\nWhen this bit is reset, frames whose SA does not match the SA\n\nregisters will be marked as failing the SA Address filter."]
pub type SaifR = crate::BitReader;
#[doc = "Field `SAIF` writer - SA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the SA address comparison. The frames whose\n\nSA matches the SA registers will be marked as failing the SA\n\nAddress filter.\n\nWhen this bit is reset, frames whose SA does not match the SA\n\nregisters will be marked as failing the SA Address filter."]
pub type SaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source Address Filter Enable\n\nThe GMAC core compares the SA field of the received frames with\n\nthe values programmed in the enabled SA registers. If the\n\ncomparison matches, then the SAMatch bit of RxStatus Word is\n\nset high. When this bit is set high and the SA filter fails, the\n\nGMAC drops the frame.\n\nWhen this bit is reset, then the GMAC Core forwards the received\n\nframe to the application and with the updated SA Match bit of the\n\nRxStatus depending on the SA address comparison."]
pub type SafR = crate::BitReader;
#[doc = "Field `SAF` writer - Source Address Filter Enable\n\nThe GMAC core compares the SA field of the received frames with\n\nthe values programmed in the enabled SA registers. If the\n\ncomparison matches, then the SAMatch bit of RxStatus Word is\n\nset high. When this bit is set high and the SA filter fails, the\n\nGMAC drops the frame.\n\nWhen this bit is reset, then the GMAC Core forwards the received\n\nframe to the application and with the updated SA Match bit of the\n\nRxStatus depending on the SA address comparison."]
pub type SafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter\n\nWhen set, this bit configures the address filter to pass a frame if\n\nit matches either the perfect filtering or the hash filtering as set\n\nby HMC or HUC bits. When low and if the HUC/HMC bit is set, the\n\nframe is passed only if it matches the Hash filter."]
pub type HpfR = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or Perfect Filter\n\nWhen set, this bit configures the address filter to pass a frame if\n\nit matches either the perfect filtering or the hash filtering as set\n\nby HMC or HUC bits. When low and if the HUC/HMC bit is set, the\n\nframe is passed only if it matches the Hash filter."]
pub type HpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - Receive All\n\nWhen this bit is set, the GMAC Receiver module passes to the\n\nApplication all frames received irrespective of whether they pass\n\nthe address filter. The result of the SA/DA filtering is updated\n\n(pass or fail) in the corresponding bits in the Receive Status\n\nWord. When this bit is reset, the Receiver module passes to the\n\nApplication only those frames that pass the SA/DA address filter."]
pub type RaR = crate::BitReader;
#[doc = "Field `RA` writer - Receive All\n\nWhen this bit is set, the GMAC Receiver module passes to the\n\nApplication all frames received irrespective of whether they pass\n\nthe address filter. The result of the SA/DA filtering is updated\n\n(pass or fail) in the corresponding bits in the Receive Status\n\nWord. When this bit is reset, the Receiver module passes to the\n\nApplication only those frames that pass the SA/DA address filter."]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode\n\nWhen this bit is set, the Address Filter module passes all\n\nincoming frames regardless of its destination or source address.\n\nThe SA/DA Filter Fails status bits of the Receive Status Word will\n\nalways be cleared when PR is set."]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast\n\nWhen set, MAC performs destination address filtering of unicast\n\nframes according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for unicast frames, that is, it compares the DA field with\n\nthe values programmed in DA registers."]
    #[inline(always)]
    pub fn huc(&self) -> HucR {
        HucR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast\n\nWhen set, MAC performs destination address filtering of received\n\nmulticast frames according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for multicast frames, that is, it compares the DA field\n\nwith the values programmed in DA registers."]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the DA address comparison for both unicast\n\nand multicast frames.\n\nWhen reset, normal filtering of frames is performed."]
    #[inline(always)]
    pub fn daif(&self) -> DaifR {
        DaifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast\n\nWhen set, this bit indicates that all received frames with a\n\nmulticast destination address (first bit in the destination address\n\nfield is '1') are passed.\n\nWhen reset, filtering of multicast frame depends on HMC bit."]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames\n\nWhen this bit is set, the AFM module filters all incoming\n\nbroadcast frames.\n\nWhen this bit is reset, the AFM module passes all received\n\nbroadcast frames."]
    #[inline(always)]
    pub fn dbf(&self) -> DbfR {
        DbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames\n\nThese bits control the forwarding of all control frames (including\n\nunicast and multicast PAUSE frames). Note that the processing of\n\nPAUSE control frames depends only on RFE of Register\n\nGMAC_FLOW_CTRL\\[2\\]."]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the SA address comparison. The frames whose\n\nSA matches the SA registers will be marked as failing the SA\n\nAddress filter.\n\nWhen this bit is reset, frames whose SA does not match the SA\n\nregisters will be marked as failing the SA Address filter."]
    #[inline(always)]
    pub fn saif(&self) -> SaifR {
        SaifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable\n\nThe GMAC core compares the SA field of the received frames with\n\nthe values programmed in the enabled SA registers. If the\n\ncomparison matches, then the SAMatch bit of RxStatus Word is\n\nset high. When this bit is set high and the SA filter fails, the\n\nGMAC drops the frame.\n\nWhen this bit is reset, then the GMAC Core forwards the received\n\nframe to the application and with the updated SA Match bit of the\n\nRxStatus depending on the SA address comparison."]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter\n\nWhen set, this bit configures the address filter to pass a frame if\n\nit matches either the perfect filtering or the hash filtering as set\n\nby HMC or HUC bits. When low and if the HUC/HMC bit is set, the\n\nframe is passed only if it matches the Hash filter."]
    #[inline(always)]
    pub fn hpf(&self) -> HpfR {
        HpfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All\n\nWhen this bit is set, the GMAC Receiver module passes to the\n\nApplication all frames received irrespective of whether they pass\n\nthe address filter. The result of the SA/DA filtering is updated\n\n(pass or fail) in the corresponding bits in the Receive Status\n\nWord. When this bit is reset, the Receiver module passes to the\n\nApplication only those frames that pass the SA/DA address filter."]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode\n\nWhen this bit is set, the Address Filter module passes all\n\nincoming frames regardless of its destination or source address.\n\nThe SA/DA Filter Fails status bits of the Receive Status Word will\n\nalways be cleared when PR is set."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<GmacMacFrmFiltSpec> {
        PrW::new(self, 0)
    }
    #[doc = "Bit 1 - Hash Unicast\n\nWhen set, MAC performs destination address filtering of unicast\n\nframes according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for unicast frames, that is, it compares the DA field with\n\nthe values programmed in DA registers."]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HucW<GmacMacFrmFiltSpec> {
        HucW::new(self, 1)
    }
    #[doc = "Bit 2 - Hash Multicast\n\nWhen set, MAC performs destination address filtering of received\n\nmulticast frames according to the hash table.\n\nWhen reset, the MAC performs a perfect destination address\n\nfiltering for multicast frames, that is, it compares the DA field\n\nwith the values programmed in DA registers."]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HmcW<GmacMacFrmFiltSpec> {
        HmcW::new(self, 2)
    }
    #[doc = "Bit 3 - DA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the DA address comparison for both unicast\n\nand multicast frames.\n\nWhen reset, normal filtering of frames is performed."]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DaifW<GmacMacFrmFiltSpec> {
        DaifW::new(self, 3)
    }
    #[doc = "Bit 4 - Pass All Multicast\n\nWhen set, this bit indicates that all received frames with a\n\nmulticast destination address (first bit in the destination address\n\nfield is '1') are passed.\n\nWhen reset, filtering of multicast frame depends on HMC bit."]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<GmacMacFrmFiltSpec> {
        PmW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames\n\nWhen this bit is set, the AFM module filters all incoming\n\nbroadcast frames.\n\nWhen this bit is reset, the AFM module passes all received\n\nbroadcast frames."]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DbfW<GmacMacFrmFiltSpec> {
        DbfW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass Control Frames\n\nThese bits control the forwarding of all control frames (including\n\nunicast and multicast PAUSE frames). Note that the processing of\n\nPAUSE control frames depends only on RFE of Register\n\nGMAC_FLOW_CTRL\\[2\\]."]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PcfW<GmacMacFrmFiltSpec> {
        PcfW::new(self, 6)
    }
    #[doc = "Bit 8 - SA Inverse Filtering\n\nWhen this bit is set, the Address Check block operates in inverse\n\nfiltering mode for the SA address comparison. The frames whose\n\nSA matches the SA registers will be marked as failing the SA\n\nAddress filter.\n\nWhen this bit is reset, frames whose SA does not match the SA\n\nregisters will be marked as failing the SA Address filter."]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SaifW<GmacMacFrmFiltSpec> {
        SaifW::new(self, 8)
    }
    #[doc = "Bit 9 - Source Address Filter Enable\n\nThe GMAC core compares the SA field of the received frames with\n\nthe values programmed in the enabled SA registers. If the\n\ncomparison matches, then the SAMatch bit of RxStatus Word is\n\nset high. When this bit is set high and the SA filter fails, the\n\nGMAC drops the frame.\n\nWhen this bit is reset, then the GMAC Core forwards the received\n\nframe to the application and with the updated SA Match bit of the\n\nRxStatus depending on the SA address comparison."]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SafW<GmacMacFrmFiltSpec> {
        SafW::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter\n\nWhen set, this bit configures the address filter to pass a frame if\n\nit matches either the perfect filtering or the hash filtering as set\n\nby HMC or HUC bits. When low and if the HUC/HMC bit is set, the\n\nframe is passed only if it matches the Hash filter."]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HpfW<GmacMacFrmFiltSpec> {
        HpfW::new(self, 10)
    }
    #[doc = "Bit 31 - Receive All\n\nWhen this bit is set, the GMAC Receiver module passes to the\n\nApplication all frames received irrespective of whether they pass\n\nthe address filter. The result of the SA/DA filtering is updated\n\n(pass or fail) in the corresponding bits in the Receive Status\n\nWord. When this bit is reset, the Receiver module passes to the\n\nApplication only those frames that pass the SA/DA address filter."]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<GmacMacFrmFiltSpec> {
        RaW::new(self, 31)
    }
}
#[doc = "MAC Frame Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mac_frm_filt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mac_frm_filt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMacFrmFiltSpec;
impl crate::RegisterSpec for GmacMacFrmFiltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mac_frm_filt::R`](R) reader structure"]
impl crate::Readable for GmacMacFrmFiltSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mac_frm_filt::W`](W) writer structure"]
impl crate::Writable for GmacMacFrmFiltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MAC_FRM_FILT to value 0"]
impl crate::Resettable for GmacMacFrmFiltSpec {
    const RESET_VALUE: u32 = 0;
}
